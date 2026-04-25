use crate::bevy::prelude::*;
use crate::config::statics::CONFIG;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UsfAuthorityViolationMode {
    #[default]
    Panic,
    Warn,
}

#[derive(Resource, Reflect, Debug, Clone, Default, PartialEq, Eq)]
#[reflect(Resource)]
pub struct UsfAuthorityDiagnostics {
    pub canonical_guard_rejections: u64,
    pub runtime_state_guard_rejections: u64,
    pub last_rejected_domain: String,
    pub last_rejected_expected_role: String,
}

impl UsfAuthorityDiagnostics {
    fn record_canonical_rejection(&mut self, domain_id: &str) {
        self.canonical_guard_rejections = self.canonical_guard_rejections.saturating_add(1);
        self.last_rejected_domain = domain_id.to_string();
        self.last_rejected_expected_role = "canonical".to_string();
    }

    fn record_runtime_state_rejection(&mut self, domain_id: &str) {
        self.runtime_state_guard_rejections = self.runtime_state_guard_rejections.saturating_add(1);
        self.last_rejected_domain = domain_id.to_string();
        self.last_rejected_expected_role = "runtime_state".to_string();
    }
}

#[derive(Message, Reflect, Debug, Clone, PartialEq, Eq)]
pub struct UsfAuthorityDiagnosticsEvent {
    pub canonical_guard_rejections: u64,
    pub runtime_state_guard_rejections: u64,
    pub last_rejected_domain: String,
    pub last_rejected_expected_role: String,
}

#[derive(Resource, Reflect, Debug, Clone, PartialEq, Eq)]
#[reflect(Resource)]
pub struct UsfAuthorityDiagnosticsExportSettings {
    pub enabled: bool,
    pub output_path: String,
    pub flush_each_write: bool,
}

impl Default for UsfAuthorityDiagnosticsExportSettings {
    fn default() -> Self {
        Self {
            enabled: CONFIG().get::<bool>("usf/authority/diagnostics_export/enabled"),
            output_path: CONFIG().get::<String>("usf/authority/diagnostics_export/output_path"),
            flush_each_write: CONFIG().get::<bool>("usf/authority/diagnostics_export/flush_each_write"),
        }
    }
}

#[derive(Resource, Reflect, Debug, Clone, Copy, Default, PartialEq, Eq)]
#[reflect(Resource)]
pub struct UsfAuthorityDiagnosticsExportState {
    pub exported_events_total: u64,
}

#[derive(Resource, Reflect, Debug, Clone, PartialEq, Eq)]
#[reflect(Resource)]
pub struct UsfWorldAuthorityContract {
    pub canonical_entity_domains: Vec<String>,
    pub runtime_state_domains: Vec<String>,
    pub violation_mode: UsfAuthorityViolationMode,
}

impl Default for UsfWorldAuthorityContract {
    fn default() -> Self {
        Self {
            canonical_entity_domains: Vec::new(),
            runtime_state_domains: Vec::new(),
            violation_mode: UsfAuthorityViolationMode::Panic,
        }
    }
}

pub fn guard_canonical_domain_with_diagnostics(
    contract: &UsfWorldAuthorityContract,
    diagnostics: Option<&mut UsfAuthorityDiagnostics>,
    domain_id: &str,
) -> bool {
    if contract.guard_canonical_domain(domain_id) {
        return true;
    }
    if let Some(diagnostics) = diagnostics {
        diagnostics.record_canonical_rejection(domain_id);
    }
    false
}

pub fn guard_runtime_state_domain_with_diagnostics(
    contract: &UsfWorldAuthorityContract,
    diagnostics: Option<&mut UsfAuthorityDiagnostics>,
    domain_id: &str,
) -> bool {
    if contract.guard_runtime_state_domain(domain_id) {
        return true;
    }
    if let Some(diagnostics) = diagnostics {
        diagnostics.record_runtime_state_rejection(domain_id);
    }
    false
}

pub(crate) fn report_usf_authority_diagnostics_system(
    diagnostics: Res<UsfAuthorityDiagnostics>,
    mut last_reported: Local<UsfAuthorityDiagnostics>,
    mut diagnostics_writer: MessageWriter<UsfAuthorityDiagnosticsEvent>,
) {
    if !diagnostics.is_changed() {
        return;
    }
    if diagnostics.canonical_guard_rejections == 0 && diagnostics.runtime_state_guard_rejections == 0 {
        return;
    }
    if *diagnostics == *last_reported {
        return;
    }

    warn!(
        "USF authority diagnostics: canonical_guard_rejections={} runtime_state_guard_rejections={} last_rejected_domain='{}' last_rejected_expected_role='{}'",
        diagnostics.canonical_guard_rejections,
        diagnostics.runtime_state_guard_rejections,
        diagnostics.last_rejected_domain,
        diagnostics.last_rejected_expected_role
    );
    diagnostics_writer.write(UsfAuthorityDiagnosticsEvent {
        canonical_guard_rejections: diagnostics.canonical_guard_rejections,
        runtime_state_guard_rejections: diagnostics.runtime_state_guard_rejections,
        last_rejected_domain: diagnostics.last_rejected_domain.clone(),
        last_rejected_expected_role: diagnostics.last_rejected_expected_role.clone(),
    });
    *last_reported = diagnostics.clone();
}

pub(crate) fn export_usf_authority_diagnostics_events_system(
    settings: Res<UsfAuthorityDiagnosticsExportSettings>,
    mut state: ResMut<UsfAuthorityDiagnosticsExportState>,
    mut diagnostics_reader: MessageReader<UsfAuthorityDiagnosticsEvent>,
) {
    let events = diagnostics_reader.read().cloned().collect::<Vec<_>>();
    if events.is_empty() || !settings.enabled {
        return;
    }
    append_authority_diagnostics_events(Path::new(settings.output_path.as_str()), events.as_slice(), settings.flush_each_write).unwrap_or_else(|error| {
        panic!(
            "USF authority diagnostics export failed: could not append to '{}': {}",
            settings.output_path, error
        )
    });
    state.exported_events_total = state.exported_events_total.saturating_add(events.len() as u64);
}

impl UsfWorldAuthorityContract {
    pub fn is_canonical_domain(&self, domain_id: &str) -> bool {
        self.canonical_entity_domains.iter().any(|value| value.eq_ignore_ascii_case(domain_id))
    }

    pub fn is_runtime_state_domain(&self, domain_id: &str) -> bool {
        self.runtime_state_domains.iter().any(|value| value.eq_ignore_ascii_case(domain_id))
    }

    pub fn guard_canonical_domain(&self, domain_id: &str) -> bool {
        if self.is_canonical_domain(domain_id) {
            return true;
        }
        match self.violation_mode {
            UsfAuthorityViolationMode::Panic => {
                panic!(
                    "USF world authority contract violation: domain '{}' is not registered as canonical authority.",
                    domain_id
                );
            }
            UsfAuthorityViolationMode::Warn => {
                warn!(
                    "USF world authority contract violation: domain '{}' is not registered as canonical authority (skipping guarded system path).",
                    domain_id
                );
            }
        }
        false
    }

    pub fn guard_runtime_state_domain(&self, domain_id: &str) -> bool {
        if self.is_runtime_state_domain(domain_id) {
            return true;
        }
        match self.violation_mode {
            UsfAuthorityViolationMode::Panic => {
                panic!(
                    "USF world authority contract violation: domain '{}' is not registered as runtime-state authority.",
                    domain_id
                );
            }
            UsfAuthorityViolationMode::Warn => {
                warn!(
                    "USF world authority contract violation: domain '{}' is not registered as runtime-state authority (skipping guarded system path).",
                    domain_id
                );
            }
        }
        false
    }
}

pub(crate) fn validate_usf_world_authority_contract_system(_contract: Res<UsfWorldAuthorityContract>) {}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistedUsfAuthorityDiagnosticsEvent {
    pub schema_version: u16,
    pub exported_unix_ms: u64,
    pub canonical_guard_rejections: u64,
    pub runtime_state_guard_rejections: u64,
    pub last_rejected_domain: String,
    pub last_rejected_expected_role: String,
}

fn append_authority_diagnostics_events(path: &Path, events: &[UsfAuthorityDiagnosticsEvent], flush_each_write: bool) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("create parent directory failed: {error}"))?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|error| format!("open output file failed: {error}"))?;

    for event in events {
        let persisted = PersistedUsfAuthorityDiagnosticsEvent {
            schema_version: 1,
            exported_unix_ms: unix_timestamp_ms(),
            canonical_guard_rejections: event.canonical_guard_rejections,
            runtime_state_guard_rejections: event.runtime_state_guard_rejections,
            last_rejected_domain: event.last_rejected_domain.clone(),
            last_rejected_expected_role: event.last_rejected_expected_role.clone(),
        };
        let line = serde_json::to_vec(&persisted).map_err(|error| format!("serialize event failed: {error}"))?;
        file.write_all(line.as_slice()).map_err(|error| format!("write event failed: {error}"))?;
        file.write_all(b"\n").map_err(|error| format!("write newline failed: {error}"))?;
    }
    if flush_each_write {
        file.flush().map_err(|error| format!("flush failed: {error}"))?;
    }
    Ok(())
}

fn unix_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .min(u64::MAX as u128) as u64
}
