pub mod aspects;
pub mod authority;
pub mod metric;
pub mod phenomenon_realizer;
pub mod scale;
pub mod script_mvp;

use crate::bevy::prelude::*;
use crate::core::orchestration::AppSet;
use authority::{
    UsfAuthorityDiagnostics, UsfAuthorityDiagnosticsEvent, UsfAuthorityDiagnosticsExportSettings, UsfAuthorityDiagnosticsExportState,
    UsfAuthorityViolationMode, UsfWorldAuthorityContract, export_usf_authority_diagnostics_events_system, report_usf_authority_diagnostics_system,
    validate_usf_world_authority_contract_system,
};
use metric::{MetricDefinition, MetricId, MetricStorageClass, MetricValueType};
use phenomenon_realizer::PhenomenonRealizerId;
use script_mvp::{UsfScriptMvpBootstrapState, bootstrap_usf_script_mvp_system};

pub(crate) struct UsfPlugin;
impl Plugin for UsfPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UsfWorldAuthorityContract>()
            .init_resource::<UsfAuthorityDiagnostics>()
            .init_resource::<UsfAuthorityDiagnosticsExportSettings>()
            .init_resource::<UsfAuthorityDiagnosticsExportState>()
            .init_resource::<UsfScriptMvpBootstrapState>()
            .add_message::<UsfAuthorityDiagnosticsEvent>()
            .add_systems(Startup, validate_usf_world_authority_contract_system.in_set(AppSet::Diagnostics))
            .add_systems(
                Startup,
                bootstrap_usf_script_mvp_system
                    .after(validate_usf_world_authority_contract_system)
                    .in_set(AppSet::Diagnostics),
            )
            .add_systems(Update, report_usf_authority_diagnostics_system.in_set(AppSet::Diagnostics))
            .add_systems(
                Update,
                export_usf_authority_diagnostics_events_system
                    .after(report_usf_authority_diagnostics_system)
                    .in_set(AppSet::Diagnostics),
            )
            .register_type::<MetricId>()
            .register_type::<MetricDefinition>()
            .register_type::<MetricValueType>()
            .register_type::<MetricStorageClass>()
            .register_type::<PhenomenonRealizerId>()
            .register_type::<UsfAuthorityDiagnostics>()
            .register_type::<UsfAuthorityDiagnosticsEvent>()
            .register_type::<UsfAuthorityDiagnosticsExportSettings>()
            .register_type::<UsfAuthorityDiagnosticsExportState>()
            .register_type::<UsfScriptMvpBootstrapState>()
            .register_type::<UsfAuthorityViolationMode>()
            .register_type::<UsfWorldAuthorityContract>();
    }
}
