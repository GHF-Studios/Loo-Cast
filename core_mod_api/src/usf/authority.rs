use crate::bevy::prelude::*;
use crate::config::statics::CONFIG;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub const USF_DOMAIN_PHENOMENON: &str = "usf.phenomenon";
pub const USF_DOMAIN_PHENOMENON_MODEL: &str = "usf.phenomenon_model";
pub const USF_DOMAIN_PARTIAL_PHENOMENON_MODEL: &str = "usf.partial_phenomenon_model";
pub const USF_DOMAIN_SUBSTRATE: &str = "usf.substrate";
pub const USF_DOMAIN_ZONE: &str = "usf.zone";
pub const USF_DOMAIN_MANIFESTATION_RUNTIME: &str = "usf.runtime.manifestation.runtime";

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
    pub derived_guard_rejections: u64,
    pub last_rejected_domain: String,
    pub last_rejected_expected_class: String,
}
impl UsfAuthorityDiagnostics {
    fn record_canonical_rejection(&mut self, domain_id: &str) {
        self.canonical_guard_rejections = self.canonical_guard_rejections.saturating_add(1);
        self.last_rejected_domain = domain_id.to_string();
        self.last_rejected_expected_class = "canonical".to_string();
    }

    fn record_derived_rejection(&mut self, domain_id: &str) {
        self.derived_guard_rejections = self.derived_guard_rejections.saturating_add(1);
        self.last_rejected_domain = domain_id.to_string();
        self.last_rejected_expected_class = "derived".to_string();
    }
}

#[derive(Message, Reflect, Debug, Clone, PartialEq, Eq)]
pub struct UsfAuthorityDiagnosticsEvent {
    pub canonical_guard_rejections: u64,
    pub derived_guard_rejections: u64,
    pub last_rejected_domain: String,
    pub last_rejected_expected_class: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistedUsfAuthorityDiagnosticsEvent {
    pub schema_version: u16,
    pub exported_unix_ms: u64,
    pub canonical_guard_rejections: u64,
    pub derived_guard_rejections: u64,
    pub last_rejected_domain: String,
    pub last_rejected_expected_class: String,
}

#[derive(Resource, Reflect, Debug, Clone, PartialEq, Eq)]
#[reflect(Resource)]
pub struct UsfWorldAuthorityContract {
    pub canonical_entity_domains: Vec<String>,
    pub derived_runtime_domains: Vec<String>,
    pub manifestation_authority_path: String,
    pub substrate_authority_path: String,
    pub zone_authority_path: String,
    pub phenomenon_authority_path: String,
    pub violation_mode: UsfAuthorityViolationMode,
}

impl Default for UsfWorldAuthorityContract {
    fn default() -> Self {
        Self {
            canonical_entity_domains: vec![
                USF_DOMAIN_PHENOMENON.to_string(),
                USF_DOMAIN_PHENOMENON_MODEL.to_string(),
                USF_DOMAIN_PARTIAL_PHENOMENON_MODEL.to_string(),
            ],
            derived_runtime_domains: vec![
                USF_DOMAIN_SUBSTRATE.to_string(),
                USF_DOMAIN_ZONE.to_string(),
                USF_DOMAIN_MANIFESTATION_RUNTIME.to_string(),
            ],
            manifestation_authority_path: "usf.zone.ZoneRealizationState".to_string(),
            substrate_authority_path: "usf.substrate.AdaptiveSubstrateStore".to_string(),
            zone_authority_path: "usf.zone.ZoneRuntimeState".to_string(),
            phenomenon_authority_path: "usf.phenomenon.{Phenomenon,PhenomenonModel,PartialPhenomenonModel}".to_string(),
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

pub fn guard_derived_domain_with_diagnostics(contract: &UsfWorldAuthorityContract, diagnostics: Option<&mut UsfAuthorityDiagnostics>, domain_id: &str) -> bool {
    if contract.guard_derived_domain(domain_id) {
        return true;
    }
    if let Some(diagnostics) = diagnostics {
        diagnostics.record_derived_rejection(domain_id);
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
    if diagnostics.canonical_guard_rejections == 0 && diagnostics.derived_guard_rejections == 0 {
        return;
    }
    if *diagnostics == *last_reported {
        return;
    }

    warn!(
        "USF authority diagnostics: canonical_guard_rejections={} derived_guard_rejections={} last_rejected_domain='{}' last_rejected_expected_class='{}'",
        diagnostics.canonical_guard_rejections,
        diagnostics.derived_guard_rejections,
        diagnostics.last_rejected_domain,
        diagnostics.last_rejected_expected_class
    );
    diagnostics_writer.write(UsfAuthorityDiagnosticsEvent {
        canonical_guard_rejections: diagnostics.canonical_guard_rejections,
        derived_guard_rejections: diagnostics.derived_guard_rejections,
        last_rejected_domain: diagnostics.last_rejected_domain.clone(),
        last_rejected_expected_class: diagnostics.last_rejected_expected_class.clone(),
    });
    *last_reported = diagnostics.clone();
}

pub(crate) fn export_usf_authority_diagnostics_events_system(
    settings: Res<UsfAuthorityDiagnosticsExportSettings>,
    mut state: ResMut<UsfAuthorityDiagnosticsExportState>,
    mut diagnostics_reader: MessageReader<UsfAuthorityDiagnosticsEvent>,
) {
    let events = diagnostics_reader.read().cloned().collect::<Vec<_>>();
    if events.is_empty() {
        return;
    }
    if !settings.enabled {
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

    pub fn is_derived_domain(&self, domain_id: &str) -> bool {
        self.derived_runtime_domains.iter().any(|value| value.eq_ignore_ascii_case(domain_id))
    }

    pub fn assert_canonical_domain(&self, domain_id: &str) {
        if self.is_canonical_domain(domain_id) {
            return;
        }
        panic!(
            "USF world authority contract violation: domain '{}' is not registered as canonical authority.",
            domain_id
        );
    }

    pub fn assert_derived_domain(&self, domain_id: &str) {
        if self.is_derived_domain(domain_id) {
            return;
        }
        panic!(
            "USF world authority contract violation: domain '{}' is not registered as derived runtime authority.",
            domain_id
        );
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
                false
            }
        }
    }

    pub fn guard_derived_domain(&self, domain_id: &str) -> bool {
        if self.is_derived_domain(domain_id) {
            return true;
        }
        match self.violation_mode {
            UsfAuthorityViolationMode::Panic => {
                panic!(
                    "USF world authority contract violation: domain '{}' is not registered as derived runtime authority.",
                    domain_id
                );
            }
            UsfAuthorityViolationMode::Warn => {
                warn!(
                    "USF world authority contract violation: domain '{}' is not registered as derived runtime authority (skipping guarded system path).",
                    domain_id
                );
                false
            }
        }
    }
}

fn append_authority_diagnostics_events(output_path: &Path, events: &[UsfAuthorityDiagnosticsEvent], flush_each_write: bool) -> Result<(), String> {
    if events.is_empty() {
        return Ok(());
    }
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("create_dir_all failed: {error}"))?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_path)
        .map_err(|error| format!("open file failed: {error}"))?;

    for event in events {
        let persisted = PersistedUsfAuthorityDiagnosticsEvent {
            schema_version: 1,
            exported_unix_ms: unix_timestamp_ms(),
            canonical_guard_rejections: event.canonical_guard_rejections,
            derived_guard_rejections: event.derived_guard_rejections,
            last_rejected_domain: event.last_rejected_domain.clone(),
            last_rejected_expected_class: event.last_rejected_expected_class.clone(),
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

pub(crate) fn validate_usf_world_authority_contract_system(contract: Res<UsfWorldAuthorityContract>) {
    if contract.canonical_entity_domains.is_empty() {
        panic!("USF world authority validation failed: canonical_entity_domains must not be empty.");
    }
    if contract.derived_runtime_domains.is_empty() {
        panic!("USF world authority validation failed: derived_runtime_domains must not be empty.");
    }

    let required_canonical = [USF_DOMAIN_PHENOMENON, USF_DOMAIN_PHENOMENON_MODEL, USF_DOMAIN_PARTIAL_PHENOMENON_MODEL];
    for required in required_canonical {
        if !contract.is_canonical_domain(required) {
            panic!("USF world authority validation failed: missing canonical entity domain '{}'.", required);
        }
    }
    let required_derived = [USF_DOMAIN_SUBSTRATE, USF_DOMAIN_ZONE, USF_DOMAIN_MANIFESTATION_RUNTIME];
    for required in required_derived {
        if !contract.is_derived_domain(required) {
            panic!("USF world authority validation failed: missing derived runtime domain '{}'.", required);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_contract_contains_required_domain_ids() {
        let contract = UsfWorldAuthorityContract::default();
        contract.assert_canonical_domain(USF_DOMAIN_PHENOMENON);
        contract.assert_canonical_domain(USF_DOMAIN_PHENOMENON_MODEL);
        contract.assert_canonical_domain(USF_DOMAIN_PARTIAL_PHENOMENON_MODEL);
        contract.assert_derived_domain(USF_DOMAIN_SUBSTRATE);
        contract.assert_derived_domain(USF_DOMAIN_ZONE);
        contract.assert_derived_domain(USF_DOMAIN_MANIFESTATION_RUNTIME);
    }

    #[test]
    fn warn_mode_guards_return_false_instead_of_panicking() {
        let mut contract = UsfWorldAuthorityContract::default();
        contract.violation_mode = UsfAuthorityViolationMode::Warn;
        assert!(!contract.guard_canonical_domain("usf.unknown.domain"));
        assert!(!contract.guard_derived_domain("usf.unknown.domain"));
    }

    #[test]
    fn diagnostics_track_guard_rejections() {
        let mut contract = UsfWorldAuthorityContract::default();
        contract.violation_mode = UsfAuthorityViolationMode::Warn;
        let mut diagnostics = UsfAuthorityDiagnostics::default();

        assert!(!guard_canonical_domain_with_diagnostics(
            &contract,
            Some(&mut diagnostics),
            "usf.invalid.canonical"
        ));
        assert!(!guard_derived_domain_with_diagnostics(&contract, Some(&mut diagnostics), "usf.invalid.derived"));

        assert_eq!(diagnostics.canonical_guard_rejections, 1);
        assert_eq!(diagnostics.derived_guard_rejections, 1);
        assert_eq!(diagnostics.last_rejected_domain, "usf.invalid.derived");
        assert_eq!(diagnostics.last_rejected_expected_class, "derived");
    }

    #[test]
    fn authority_diagnostics_events_can_be_exported_as_ndjson() {
        let temp_path = std::env::temp_dir().join(format!("usf_authority_diag_{:x}.ndjson", unix_timestamp_ms()));
        let events = vec![
            UsfAuthorityDiagnosticsEvent {
                canonical_guard_rejections: 1,
                derived_guard_rejections: 0,
                last_rejected_domain: "usf.invalid.canonical".to_string(),
                last_rejected_expected_class: "canonical".to_string(),
            },
            UsfAuthorityDiagnosticsEvent {
                canonical_guard_rejections: 1,
                derived_guard_rejections: 2,
                last_rejected_domain: "usf.invalid.derived".to_string(),
                last_rejected_expected_class: "derived".to_string(),
            },
        ];
        append_authority_diagnostics_events(temp_path.as_path(), events.as_slice(), true).expect("append diagnostics events");

        let bytes = std::fs::read(&temp_path).expect("read ndjson output");
        let text = String::from_utf8(bytes).expect("utf8");
        let lines = text.lines().collect::<Vec<_>>();
        assert_eq!(lines.len(), 2);
        let first = serde_json::from_str::<PersistedUsfAuthorityDiagnosticsEvent>(lines[0]).expect("parse first line");
        let second = serde_json::from_str::<PersistedUsfAuthorityDiagnosticsEvent>(lines[1]).expect("parse second line");
        assert_eq!(first.canonical_guard_rejections, 1);
        assert_eq!(second.derived_guard_rejections, 2);

        let _ = std::fs::remove_file(temp_path);
    }
}
