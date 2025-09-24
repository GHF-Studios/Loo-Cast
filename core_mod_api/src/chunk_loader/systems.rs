use crate::usf::scale::Scale;
use bevy::prelude::*;
use core_mod_macros::{composite_workflow, composite_workflow_return};
use tokio::task::JoinHandle;

use crate::chunk_loader::resources::RemovedChunkLoaders;
use crate::workflow::composite_workflow_context::ScopedCompositeWorkflowContext;
use crate::workflow::functions::handle_composite_workflow_return_now;

#[tracing::instrument(skip_all)]
pub(crate) fn update_chunk_loader_system(mut composite_workflow_handle: Local<Option<JoinHandle<ScopedCompositeWorkflowContext>>>) {
    let handle_is_some = (*composite_workflow_handle).is_some();
    let handle_is_finished = match *composite_workflow_handle {
        Some(ref handle) => handle.is_finished(),
        None => false,
    };

    if !handle_is_some {
        let handle = composite_workflow!(UpdateChunkLoaders, {
            let categorize_chunks_output = workflow!(O, ChunkLoader::CategorizeChunks);

            let load_chunk_inputs_scale_quecto_meter_000001 = categorize_chunks_output.inner_scale_quecto_meter_000001.load_chunk_inputs;
            let unload_chunk_inputs_scale_quecto_meter_000001 = categorize_chunks_output.inner_scale_quecto_meter_000001.unload_chunk_inputs;

            let load_chunk_inputs_scale_quecto_meter_00001 = categorize_chunks_output.inner_scale_quecto_meter_00001.load_chunk_inputs;
            let unload_chunk_inputs_scale_quecto_meter_00001 = categorize_chunks_output.inner_scale_quecto_meter_00001.unload_chunk_inputs;

            let load_chunk_inputs_scale_quecto_meter_0001 = categorize_chunks_output.inner_scale_quecto_meter_0001.load_chunk_inputs;
            let unload_chunk_inputs_scale_quecto_meter_0001 = categorize_chunks_output.inner_scale_quecto_meter_0001.unload_chunk_inputs;

            let load_chunk_inputs_scale_quecto_meter_001 = categorize_chunks_output.inner_scale_quecto_meter_001.load_chunk_inputs;
            let unload_chunk_inputs_scale_quecto_meter_001 = categorize_chunks_output.inner_scale_quecto_meter_001.unload_chunk_inputs;

            let load_chunk_inputs_scale_quecto_meter_01 = categorize_chunks_output.inner_scale_quecto_meter_01.load_chunk_inputs;
            let unload_chunk_inputs_scale_quecto_meter_01 = categorize_chunks_output.inner_scale_quecto_meter_01.unload_chunk_inputs;

            let load_chunk_inputs_scale_quecto_meter_1 = categorize_chunks_output.inner_scale_quecto_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_quecto_meter_1 = categorize_chunks_output.inner_scale_quecto_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_quecto_meter_10 = categorize_chunks_output.inner_scale_quecto_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_quecto_meter_10 = categorize_chunks_output.inner_scale_quecto_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_quecto_meter_100 = categorize_chunks_output.inner_scale_quecto_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_quecto_meter_100 = categorize_chunks_output.inner_scale_quecto_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_ronto_meter_1 = categorize_chunks_output.inner_scale_ronto_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_ronto_meter_1 = categorize_chunks_output.inner_scale_ronto_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_ronto_meter_10 = categorize_chunks_output.inner_scale_ronto_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_ronto_meter_10 = categorize_chunks_output.inner_scale_ronto_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_ronto_meter_100 = categorize_chunks_output.inner_scale_ronto_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_ronto_meter_100 = categorize_chunks_output.inner_scale_ronto_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_yocto_meter_1 = categorize_chunks_output.inner_scale_yocto_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_yocto_meter_1 = categorize_chunks_output.inner_scale_yocto_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_yocto_meter_10 = categorize_chunks_output.inner_scale_yocto_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_yocto_meter_10 = categorize_chunks_output.inner_scale_yocto_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_yocto_meter_100 = categorize_chunks_output.inner_scale_yocto_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_yocto_meter_100 = categorize_chunks_output.inner_scale_yocto_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_zepto_meter_1 = categorize_chunks_output.inner_scale_zepto_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_zepto_meter_1 = categorize_chunks_output.inner_scale_zepto_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_zepto_meter_10 = categorize_chunks_output.inner_scale_zepto_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_zepto_meter_10 = categorize_chunks_output.inner_scale_zepto_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_zepto_meter_100 = categorize_chunks_output.inner_scale_zepto_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_zepto_meter_100 = categorize_chunks_output.inner_scale_zepto_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_atto_meter_1 = categorize_chunks_output.inner_scale_atto_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_atto_meter_1 = categorize_chunks_output.inner_scale_atto_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_atto_meter_10 = categorize_chunks_output.inner_scale_atto_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_atto_meter_10 = categorize_chunks_output.inner_scale_atto_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_atto_meter_100 = categorize_chunks_output.inner_scale_atto_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_atto_meter_100 = categorize_chunks_output.inner_scale_atto_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_femto_meter_1 = categorize_chunks_output.inner_scale_femto_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_femto_meter_1 = categorize_chunks_output.inner_scale_femto_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_femto_meter_10 = categorize_chunks_output.inner_scale_femto_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_femto_meter_10 = categorize_chunks_output.inner_scale_femto_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_femto_meter_100 = categorize_chunks_output.inner_scale_femto_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_femto_meter_100 = categorize_chunks_output.inner_scale_femto_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_pico_meter_1 = categorize_chunks_output.inner_scale_pico_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_pico_meter_1 = categorize_chunks_output.inner_scale_pico_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_pico_meter_10 = categorize_chunks_output.inner_scale_pico_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_pico_meter_10 = categorize_chunks_output.inner_scale_pico_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_pico_meter_100 = categorize_chunks_output.inner_scale_pico_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_pico_meter_100 = categorize_chunks_output.inner_scale_pico_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_nano_meter_1 = categorize_chunks_output.inner_scale_nano_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_nano_meter_1 = categorize_chunks_output.inner_scale_nano_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_nano_meter_10 = categorize_chunks_output.inner_scale_nano_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_nano_meter_10 = categorize_chunks_output.inner_scale_nano_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_nano_meter_100 = categorize_chunks_output.inner_scale_nano_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_nano_meter_100 = categorize_chunks_output.inner_scale_nano_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_micro_meter_1 = categorize_chunks_output.inner_scale_micro_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_micro_meter_1 = categorize_chunks_output.inner_scale_micro_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_micro_meter_10 = categorize_chunks_output.inner_scale_micro_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_micro_meter_10 = categorize_chunks_output.inner_scale_micro_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_micro_meter_100 = categorize_chunks_output.inner_scale_micro_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_micro_meter_100 = categorize_chunks_output.inner_scale_micro_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_milli_meter_1 = categorize_chunks_output.inner_scale_milli_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_milli_meter_1 = categorize_chunks_output.inner_scale_milli_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_milli_meter_10 = categorize_chunks_output.inner_scale_milli_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_milli_meter_10 = categorize_chunks_output.inner_scale_milli_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_milli_meter_100 = categorize_chunks_output.inner_scale_milli_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_milli_meter_100 = categorize_chunks_output.inner_scale_milli_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_meter_1 = categorize_chunks_output.inner_scale_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_meter_1 = categorize_chunks_output.inner_scale_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_meter_10 = categorize_chunks_output.inner_scale_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_meter_10 = categorize_chunks_output.inner_scale_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_meter_100 = categorize_chunks_output.inner_scale_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_meter_100 = categorize_chunks_output.inner_scale_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_kilo_meter_1 = categorize_chunks_output.inner_scale_kilo_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_kilo_meter_1 = categorize_chunks_output.inner_scale_kilo_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_kilo_meter_10 = categorize_chunks_output.inner_scale_kilo_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_kilo_meter_10 = categorize_chunks_output.inner_scale_kilo_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_kilo_meter_100 = categorize_chunks_output.inner_scale_kilo_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_kilo_meter_100 = categorize_chunks_output.inner_scale_kilo_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_mega_meter_1 = categorize_chunks_output.inner_scale_mega_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_mega_meter_1 = categorize_chunks_output.inner_scale_mega_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_mega_meter_10 = categorize_chunks_output.inner_scale_mega_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_mega_meter_10 = categorize_chunks_output.inner_scale_mega_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_mega_meter_100 = categorize_chunks_output.inner_scale_mega_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_mega_meter_100 = categorize_chunks_output.inner_scale_mega_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_giga_meter_1 = categorize_chunks_output.inner_scale_giga_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_giga_meter_1 = categorize_chunks_output.inner_scale_giga_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_giga_meter_10 = categorize_chunks_output.inner_scale_giga_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_giga_meter_10 = categorize_chunks_output.inner_scale_giga_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_giga_meter_100 = categorize_chunks_output.inner_scale_giga_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_giga_meter_100 = categorize_chunks_output.inner_scale_giga_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_tera_meter_1 = categorize_chunks_output.inner_scale_tera_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_tera_meter_1 = categorize_chunks_output.inner_scale_tera_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_tera_meter_10 = categorize_chunks_output.inner_scale_tera_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_tera_meter_10 = categorize_chunks_output.inner_scale_tera_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_tera_meter_100 = categorize_chunks_output.inner_scale_tera_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_tera_meter_100 = categorize_chunks_output.inner_scale_tera_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_peta_meter_1 = categorize_chunks_output.inner_scale_peta_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_peta_meter_1 = categorize_chunks_output.inner_scale_peta_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_peta_meter_10 = categorize_chunks_output.inner_scale_peta_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_peta_meter_10 = categorize_chunks_output.inner_scale_peta_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_peta_meter_100 = categorize_chunks_output.inner_scale_peta_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_peta_meter_100 = categorize_chunks_output.inner_scale_peta_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_exa_meter_1 = categorize_chunks_output.inner_scale_exa_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_exa_meter_1 = categorize_chunks_output.inner_scale_exa_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_exa_meter_10 = categorize_chunks_output.inner_scale_exa_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_exa_meter_10 = categorize_chunks_output.inner_scale_exa_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_exa_meter_100 = categorize_chunks_output.inner_scale_exa_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_exa_meter_100 = categorize_chunks_output.inner_scale_exa_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_zetta_meter_1 = categorize_chunks_output.inner_scale_zetta_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_zetta_meter_1 = categorize_chunks_output.inner_scale_zetta_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_zetta_meter_10 = categorize_chunks_output.inner_scale_zetta_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_zetta_meter_10 = categorize_chunks_output.inner_scale_zetta_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_zetta_meter_100 = categorize_chunks_output.inner_scale_zetta_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_zetta_meter_100 = categorize_chunks_output.inner_scale_zetta_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_yotta_meter_1 = categorize_chunks_output.inner_scale_yotta_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_yotta_meter_1 = categorize_chunks_output.inner_scale_yotta_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_yotta_meter_10 = categorize_chunks_output.inner_scale_yotta_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_yotta_meter_10 = categorize_chunks_output.inner_scale_yotta_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_yotta_meter_100 = categorize_chunks_output.inner_scale_yotta_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_yotta_meter_100 = categorize_chunks_output.inner_scale_yotta_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_ronna_meter_1 = categorize_chunks_output.inner_scale_ronna_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_ronna_meter_1 = categorize_chunks_output.inner_scale_ronna_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_ronna_meter_10 = categorize_chunks_output.inner_scale_ronna_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_ronna_meter_10 = categorize_chunks_output.inner_scale_ronna_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_ronna_meter_100 = categorize_chunks_output.inner_scale_ronna_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_ronna_meter_100 = categorize_chunks_output.inner_scale_ronna_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_quetta_meter_1 = categorize_chunks_output.inner_scale_quetta_meter_1.load_chunk_inputs;
            let unload_chunk_inputs_scale_quetta_meter_1 = categorize_chunks_output.inner_scale_quetta_meter_1.unload_chunk_inputs;

            let load_chunk_inputs_scale_quetta_meter_10 = categorize_chunks_output.inner_scale_quetta_meter_10.load_chunk_inputs;
            let unload_chunk_inputs_scale_quetta_meter_10 = categorize_chunks_output.inner_scale_quetta_meter_10.unload_chunk_inputs;

            let load_chunk_inputs_scale_quetta_meter_100 = categorize_chunks_output.inner_scale_quetta_meter_100.load_chunk_inputs;
            let unload_chunk_inputs_scale_quetta_meter_100 = categorize_chunks_output.inner_scale_quetta_meter_100.unload_chunk_inputs;

            let load_chunk_inputs_scale_quetta_meter_1000 = categorize_chunks_output.inner_scale_quetta_meter_1000.load_chunk_inputs;
            let unload_chunk_inputs_scale_quetta_meter_1000 = categorize_chunks_output.inner_scale_quetta_meter_1000.unload_chunk_inputs;

            let load_chunk_inputs_scale_quetta_meter_10000 = categorize_chunks_output.inner_scale_quetta_meter_10000.load_chunk_inputs;
            let unload_chunk_inputs_scale_quetta_meter_10000 = categorize_chunks_output.inner_scale_quetta_meter_10000.unload_chunk_inputs;

            let load_chunk_inputs_scale_quetta_meter_100000 = categorize_chunks_output.inner_scale_quetta_meter_100000.load_chunk_inputs;
            let unload_chunk_inputs_scale_quetta_meter_100000 = categorize_chunks_output.inner_scale_quetta_meter_100000.unload_chunk_inputs;

            workflow!(I, ChunkLoader::LoadChunks, Input {
                inner_scale_quecto_meter_000001: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_quecto_meter_000001 },
                inner_scale_quecto_meter_00001: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_quecto_meter_00001 },
                inner_scale_quecto_meter_0001: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_quecto_meter_0001 },
                inner_scale_quecto_meter_001: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_quecto_meter_001 },
                inner_scale_quecto_meter_01: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_quecto_meter_01 },
                inner_scale_quecto_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_quecto_meter_1 },
                inner_scale_quecto_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_quecto_meter_10 },
                inner_scale_quecto_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_quecto_meter_100 },
                inner_scale_ronto_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_ronto_meter_1 },
                inner_scale_ronto_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_ronto_meter_10 },
                inner_scale_ronto_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_ronto_meter_100 },
                inner_scale_yocto_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_yocto_meter_1 },
                inner_scale_yocto_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_yocto_meter_10 },
                inner_scale_yocto_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_yocto_meter_100 },
                inner_scale_zepto_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_zepto_meter_1 },
                inner_scale_zepto_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_zepto_meter_10 },
                inner_scale_zepto_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_zepto_meter_100 },
                inner_scale_atto_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_atto_meter_1 },
                inner_scale_atto_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_atto_meter_10 },
                inner_scale_atto_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_atto_meter_100 },
                inner_scale_femto_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_femto_meter_1 },
                inner_scale_femto_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_femto_meter_10 },
                inner_scale_femto_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_femto_meter_100 },
                inner_scale_pico_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_pico_meter_1 },
                inner_scale_pico_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_pico_meter_10 },
                inner_scale_pico_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_pico_meter_100 },
                inner_scale_nano_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_nano_meter_1 },
                inner_scale_nano_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_nano_meter_10 },
                inner_scale_nano_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_nano_meter_100 },
                inner_scale_micro_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_micro_meter_1 },
                inner_scale_micro_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_micro_meter_10 },
                inner_scale_micro_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_micro_meter_100 },
                inner_scale_milli_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_milli_meter_1 },
                inner_scale_milli_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_milli_meter_10 },
                inner_scale_milli_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_milli_meter_100 },
                inner_scale_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_meter_1 },
                inner_scale_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_meter_10 },
                inner_scale_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_meter_100 },
                inner_scale_kilo_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_kilo_meter_1 },
                inner_scale_kilo_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_kilo_meter_10 },
                inner_scale_kilo_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_kilo_meter_100 },
                inner_scale_mega_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_mega_meter_1 },
                inner_scale_mega_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_mega_meter_10 },
                inner_scale_mega_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_mega_meter_100 },
                inner_scale_giga_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_giga_meter_1 },
                inner_scale_giga_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_giga_meter_10 },
                inner_scale_giga_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_giga_meter_100 },
                inner_scale_tera_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_tera_meter_1 },
                inner_scale_tera_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_tera_meter_10 },
                inner_scale_tera_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_tera_meter_100 },
                inner_scale_peta_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_peta_meter_1 },
                inner_scale_peta_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_peta_meter_10 },
                inner_scale_peta_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_peta_meter_100 },
                inner_scale_exa_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_exa_meter_1 },
                inner_scale_exa_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_exa_meter_10 },
                inner_scale_exa_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_exa_meter_100 },
                inner_scale_zetta_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_zetta_meter_1 },
                inner_scale_zetta_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_zetta_meter_10 },
                inner_scale_zetta_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_zetta_meter_100 },
                inner_scale_yotta_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_yotta_meter_1 },
                inner_scale_yotta_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_yotta_meter_10 },
                inner_scale_yotta_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_yotta_meter_100 },
                inner_scale_ronna_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_ronna_meter_1 },
                inner_scale_ronna_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_ronna_meter_10 },
                inner_scale_ronna_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_ronna_meter_100 },
                inner_scale_quetta_meter_1: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_quetta_meter_1 },
                inner_scale_quetta_meter_10: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_quetta_meter_10 },
                inner_scale_quetta_meter_100: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_quetta_meter_100 },
                inner_scale_quetta_meter_1000: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_quetta_meter_1000 },
                inner_scale_quetta_meter_10000: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_quetta_meter_10000 },
                inner_scale_quetta_meter_100000: crate::chunk_loader::workflows::external::load_chunks::Input { inputs: load_chunk_inputs_scale_quetta_meter_100000 },
            });

            workflow!(I, ChunkLoader::UnloadChunks, Input {
                inner_scale_quecto_meter_000001: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_quecto_meter_000001 },
                inner_scale_quecto_meter_00001: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_quecto_meter_00001 },
                inner_scale_quecto_meter_0001: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_quecto_meter_0001 },
                inner_scale_quecto_meter_001: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_quecto_meter_001 },
                inner_scale_quecto_meter_01: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_quecto_meter_01 },
                inner_scale_quecto_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_quecto_meter_1 },
                inner_scale_quecto_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_quecto_meter_10 },
                inner_scale_quecto_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_quecto_meter_100 },
                inner_scale_ronto_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_ronto_meter_1 },
                inner_scale_ronto_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_ronto_meter_10 },
                inner_scale_ronto_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_ronto_meter_100 },
                inner_scale_yocto_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_yocto_meter_1 },
                inner_scale_yocto_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_yocto_meter_10 },
                inner_scale_yocto_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_yocto_meter_100 },
                inner_scale_zepto_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_zepto_meter_1 },
                inner_scale_zepto_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_zepto_meter_10 },
                inner_scale_zepto_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_zepto_meter_100 },
                inner_scale_atto_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_atto_meter_1 },
                inner_scale_atto_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_atto_meter_10 },
                inner_scale_atto_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_atto_meter_100 },
                inner_scale_femto_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_femto_meter_1 },
                inner_scale_femto_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_femto_meter_10 },
                inner_scale_femto_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_femto_meter_100 },
                inner_scale_pico_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_pico_meter_1 },
                inner_scale_pico_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_pico_meter_10 },
                inner_scale_pico_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_pico_meter_100 },
                inner_scale_nano_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_nano_meter_1 },
                inner_scale_nano_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_nano_meter_10 },
                inner_scale_nano_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_nano_meter_100 },
                inner_scale_micro_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_micro_meter_1 },
                inner_scale_micro_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_micro_meter_10 },
                inner_scale_micro_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_micro_meter_100 },
                inner_scale_milli_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_milli_meter_1 },
                inner_scale_milli_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_milli_meter_10 },
                inner_scale_milli_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_milli_meter_100 },
                inner_scale_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_meter_1 },
                inner_scale_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_meter_10 },
                inner_scale_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_meter_100 },
                inner_scale_kilo_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_kilo_meter_1 },
                inner_scale_kilo_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_kilo_meter_10 },
                inner_scale_kilo_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_kilo_meter_100 },
                inner_scale_mega_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_mega_meter_1 },
                inner_scale_mega_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_mega_meter_10 },
                inner_scale_mega_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_mega_meter_100 },
                inner_scale_giga_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_giga_meter_1 },
                inner_scale_giga_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_giga_meter_10 },
                inner_scale_giga_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_giga_meter_100 },
                inner_scale_tera_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_tera_meter_1 },
                inner_scale_tera_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_tera_meter_10 },
                inner_scale_tera_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_tera_meter_100 },
                inner_scale_peta_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_peta_meter_1 },
                inner_scale_peta_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_peta_meter_10 },
                inner_scale_peta_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_peta_meter_100 },
                inner_scale_exa_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_exa_meter_1 },
                inner_scale_exa_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_exa_meter_10 },
                inner_scale_exa_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_exa_meter_100 },
                inner_scale_zetta_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_zetta_meter_1 },
                inner_scale_zetta_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_zetta_meter_10 },
                inner_scale_zetta_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_zetta_meter_100 },
                inner_scale_yotta_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_yotta_meter_1 },
                inner_scale_yotta_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_yotta_meter_10 },
                inner_scale_yotta_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_yotta_meter_100 },
                inner_scale_ronna_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_ronna_meter_1 },
                inner_scale_ronna_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_ronna_meter_10 },
                inner_scale_ronna_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_ronna_meter_100 },
                inner_scale_quetta_meter_1: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_quetta_meter_1 },
                inner_scale_quetta_meter_10: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_quetta_meter_10 },
                inner_scale_quetta_meter_100: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_quetta_meter_100 },
                inner_scale_quetta_meter_1000: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_quetta_meter_1000 },
                inner_scale_quetta_meter_10000: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_quetta_meter_10000 },
                inner_scale_quetta_meter_100000: crate::chunk_loader::workflows::external::unload_chunks::Input { inputs: unload_chunk_inputs_scale_quetta_meter_100000 },
            });
        });

        *composite_workflow_handle = Some(handle);
    }
    if handle_is_some && !handle_is_finished {
        return;
    }

    if handle_is_some && handle_is_finished {
        let handle = composite_workflow_handle.take().unwrap();
        handle_composite_workflow_return_now(handle, |_ctx| {
            composite_workflow_return!();
        });
    }
}

#[tracing::instrument(skip_all)]
pub(crate) fn post_update_chunk_loader_system<S: Scale>(mut removed_chunk_loaders: ResMut<RemovedChunkLoaders<S>>) {
    removed_chunk_loaders.0.clear();
}
