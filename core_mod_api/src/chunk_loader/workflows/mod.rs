pub mod external;

use core_mod_macros::define_workflow_mod_OLD;

define_workflow_mod_OLD! {
    name: "ChunkLoader",
    workflows: [
        CategorizeChunks, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use bevy::prelude::ResMut;

                use crate::chunk_loader::workflows::external::categorize_chunks::{
                    MainAccess as CategorizeStageMainAccess,
                    Output as CategorizeStageOutput,
                    run_ecs as categorize_stage_run_ecs
                };
                use crate::usf::scale::*;
            },
            user_items: {
            },
            stages: [
                Categorize: Ecs, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            inner_scale_quecto_meter_000001: CategorizeStageMainAccess<'w, 's, ScaleQuectoMeter000001>,
                            inner_scale_quecto_meter_00001: CategorizeStageMainAccess<'w, 's, ScaleQuectoMeter00001>,
                            inner_scale_quecto_meter_0001: CategorizeStageMainAccess<'w, 's, ScaleQuectoMeter0001>,
                            inner_scale_quecto_meter_001: CategorizeStageMainAccess<'w, 's, ScaleQuectoMeter001>,
                            inner_scale_quecto_meter_01: CategorizeStageMainAccess<'w, 's, ScaleQuectoMeter01>,
                            inner_scale_quecto_meter_1: CategorizeStageMainAccess<'w, 's, ScaleQuectoMeter1>,
                            inner_scale_quecto_meter_10: CategorizeStageMainAccess<'w, 's, ScaleQuectoMeter10>,
                            inner_scale_quecto_meter_100: CategorizeStageMainAccess<'w, 's, ScaleQuectoMeter100>,
                            inner_scale_ronto_meter_1: CategorizeStageMainAccess<'w, 's, ScaleRontoMeter1>,
                            inner_scale_ronto_meter_10: CategorizeStageMainAccess<'w, 's, ScaleRontoMeter10>,
                            inner_scale_ronto_meter_100: CategorizeStageMainAccess<'w, 's, ScaleRontoMeter100>,
                            inner_scale_yocto_meter_1: CategorizeStageMainAccess<'w, 's, ScaleYoctoMeter1>,
                            inner_scale_yocto_meter_10: CategorizeStageMainAccess<'w, 's, ScaleYoctoMeter10>,
                            inner_scale_yocto_meter_100: CategorizeStageMainAccess<'w, 's, ScaleYoctoMeter100>,
                            inner_scale_zepto_meter_1: CategorizeStageMainAccess<'w, 's, ScaleZeptoMeter1>,
                            inner_scale_zepto_meter_10: CategorizeStageMainAccess<'w, 's, ScaleZeptoMeter10>,
                            inner_scale_zepto_meter_100: CategorizeStageMainAccess<'w, 's, ScaleZeptoMeter100>,
                            inner_scale_atto_meter_1: CategorizeStageMainAccess<'w, 's, ScaleAttoMeter1>,
                            inner_scale_atto_meter_10: CategorizeStageMainAccess<'w, 's, ScaleAttoMeter10>,
                            inner_scale_atto_meter_100: CategorizeStageMainAccess<'w, 's, ScaleAttoMeter100>,
                            inner_scale_femto_meter_1: CategorizeStageMainAccess<'w, 's, ScaleFemtoMeter1>,
                            inner_scale_femto_meter_10: CategorizeStageMainAccess<'w, 's, ScaleFemtoMeter10>,
                            inner_scale_femto_meter_100: CategorizeStageMainAccess<'w, 's, ScaleFemtoMeter100>,
                            inner_scale_pico_meter_1: CategorizeStageMainAccess<'w, 's, ScalePicoMeter1>,
                            inner_scale_pico_meter_10: CategorizeStageMainAccess<'w, 's, ScalePicoMeter10>,
                            inner_scale_pico_meter_100: CategorizeStageMainAccess<'w, 's, ScalePicoMeter100>,
                            inner_scale_nano_meter_1: CategorizeStageMainAccess<'w, 's, ScaleNanoMeter1>,
                            inner_scale_nano_meter_10: CategorizeStageMainAccess<'w, 's, ScaleNanoMeter10>,
                            inner_scale_nano_meter_100: CategorizeStageMainAccess<'w, 's, ScaleNanoMeter100>,
                            inner_scale_micro_meter_1: CategorizeStageMainAccess<'w, 's, ScaleMicroMeter1>,
                            inner_scale_micro_meter_10: CategorizeStageMainAccess<'w, 's, ScaleMicroMeter10>,
                            inner_scale_micro_meter_100: CategorizeStageMainAccess<'w, 's, ScaleMicroMeter100>,
                            inner_scale_milli_meter_1: CategorizeStageMainAccess<'w, 's, ScaleMilliMeter1>,
                            inner_scale_milli_meter_10: CategorizeStageMainAccess<'w, 's, ScaleMilliMeter10>,
                            inner_scale_milli_meter_100: CategorizeStageMainAccess<'w, 's, ScaleMilliMeter100>,
                            inner_scale_meter_1: CategorizeStageMainAccess<'w, 's, ScaleMeter1>,
                            inner_scale_meter_10: CategorizeStageMainAccess<'w, 's, ScaleMeter10>,
                            inner_scale_meter_100: CategorizeStageMainAccess<'w, 's, ScaleMeter100>,
                            inner_scale_kilo_meter_1: CategorizeStageMainAccess<'w, 's, ScaleKiloMeter1>,
                            inner_scale_kilo_meter_10: CategorizeStageMainAccess<'w, 's, ScaleKiloMeter10>,
                            inner_scale_kilo_meter_100: CategorizeStageMainAccess<'w, 's, ScaleKiloMeter100>,
                            inner_scale_mega_meter_1: CategorizeStageMainAccess<'w, 's, ScaleMegaMeter1>,
                            inner_scale_mega_meter_10: CategorizeStageMainAccess<'w, 's, ScaleMegaMeter10>,
                            inner_scale_mega_meter_100: CategorizeStageMainAccess<'w, 's, ScaleMegaMeter100>,
                            inner_scale_giga_meter_1: CategorizeStageMainAccess<'w, 's, ScaleGigaMeter1>,
                            inner_scale_giga_meter_10: CategorizeStageMainAccess<'w, 's, ScaleGigaMeter10>,
                            inner_scale_giga_meter_100: CategorizeStageMainAccess<'w, 's, ScaleGigaMeter100>,
                            inner_scale_tera_meter_1: CategorizeStageMainAccess<'w, 's, ScaleTeraMeter1>,
                            inner_scale_tera_meter_10: CategorizeStageMainAccess<'w, 's, ScaleTeraMeter10>,
                            inner_scale_tera_meter_100: CategorizeStageMainAccess<'w, 's, ScaleTeraMeter100>,
                            inner_scale_peta_meter_1: CategorizeStageMainAccess<'w, 's, ScalePetaMeter1>,
                            inner_scale_peta_meter_10: CategorizeStageMainAccess<'w, 's, ScalePetaMeter10>,
                            inner_scale_peta_meter_100: CategorizeStageMainAccess<'w, 's, ScalePetaMeter100>,
                            inner_scale_exa_meter_1: CategorizeStageMainAccess<'w, 's, ScaleExaMeter1>,
                            inner_scale_exa_meter_10: CategorizeStageMainAccess<'w, 's, ScaleExaMeter10>,
                            inner_scale_exa_meter_100: CategorizeStageMainAccess<'w, 's, ScaleExaMeter100>,
                            inner_scale_zetta_meter_1: CategorizeStageMainAccess<'w, 's, ScaleZettaMeter1>,
                            inner_scale_zetta_meter_10: CategorizeStageMainAccess<'w, 's, ScaleZettaMeter10>,
                            inner_scale_zetta_meter_100: CategorizeStageMainAccess<'w, 's, ScaleZettaMeter100>,
                            inner_scale_yotta_meter_1: CategorizeStageMainAccess<'w, 's, ScaleYottaMeter1>,
                            inner_scale_yotta_meter_10: CategorizeStageMainAccess<'w, 's, ScaleYottaMeter10>,
                            inner_scale_yotta_meter_100: CategorizeStageMainAccess<'w, 's, ScaleYottaMeter100>,
                            inner_scale_ronna_meter_1: CategorizeStageMainAccess<'w, 's, ScaleRonnaMeter1>,
                            inner_scale_ronna_meter_10: CategorizeStageMainAccess<'w, 's, ScaleRonnaMeter10>,
                            inner_scale_ronna_meter_100: CategorizeStageMainAccess<'w, 's, ScaleRonnaMeter100>,
                            inner_scale_quetta_meter_1: CategorizeStageMainAccess<'w, 's, ScaleQuettaMeter1>,
                            inner_scale_quetta_meter_10: CategorizeStageMainAccess<'w, 's, ScaleQuettaMeter10>,
                            inner_scale_quetta_meter_100: CategorizeStageMainAccess<'w, 's, ScaleQuettaMeter100>,
                            inner_scale_quetta_meter_1000: CategorizeStageMainAccess<'w, 's, ScaleQuettaMeter1000>,
                            inner_scale_quetta_meter_10000: CategorizeStageMainAccess<'w, 's, ScaleQuettaMeter10000>,
                            inner_scale_quetta_meter_100000: CategorizeStageMainAccess<'w, 's, ScaleQuettaMeter100000>,
                        }

                        struct Output {
                            inner_scale_quecto_meter_000001: CategorizeStageOutput<ScaleQuectoMeter000001>,
                            inner_scale_quecto_meter_00001: CategorizeStageOutput<ScaleQuectoMeter00001>,
                            inner_scale_quecto_meter_0001: CategorizeStageOutput<ScaleQuectoMeter0001>,
                            inner_scale_quecto_meter_001: CategorizeStageOutput<ScaleQuectoMeter001>,
                            inner_scale_quecto_meter_01: CategorizeStageOutput<ScaleQuectoMeter01>,
                            inner_scale_quecto_meter_1: CategorizeStageOutput<ScaleQuectoMeter1>,
                            inner_scale_quecto_meter_10: CategorizeStageOutput<ScaleQuectoMeter10>,
                            inner_scale_quecto_meter_100: CategorizeStageOutput<ScaleQuectoMeter100>,
                            inner_scale_ronto_meter_1: CategorizeStageOutput<ScaleRontoMeter1>,
                            inner_scale_ronto_meter_10: CategorizeStageOutput<ScaleRontoMeter10>,
                            inner_scale_ronto_meter_100: CategorizeStageOutput<ScaleRontoMeter100>,
                            inner_scale_yocto_meter_1: CategorizeStageOutput<ScaleYoctoMeter1>,
                            inner_scale_yocto_meter_10: CategorizeStageOutput<ScaleYoctoMeter10>,
                            inner_scale_yocto_meter_100: CategorizeStageOutput<ScaleYoctoMeter100>,
                            inner_scale_zepto_meter_1: CategorizeStageOutput<ScaleZeptoMeter1>,
                            inner_scale_zepto_meter_10: CategorizeStageOutput<ScaleZeptoMeter10>,
                            inner_scale_zepto_meter_100: CategorizeStageOutput<ScaleZeptoMeter100>,
                            inner_scale_atto_meter_1: CategorizeStageOutput<ScaleAttoMeter1>,
                            inner_scale_atto_meter_10: CategorizeStageOutput<ScaleAttoMeter10>,
                            inner_scale_atto_meter_100: CategorizeStageOutput<ScaleAttoMeter100>,
                            inner_scale_femto_meter_1: CategorizeStageOutput<ScaleFemtoMeter1>,
                            inner_scale_femto_meter_10: CategorizeStageOutput<ScaleFemtoMeter10>,
                            inner_scale_femto_meter_100: CategorizeStageOutput<ScaleFemtoMeter100>,
                            inner_scale_pico_meter_1: CategorizeStageOutput<ScalePicoMeter1>,
                            inner_scale_pico_meter_10: CategorizeStageOutput<ScalePicoMeter10>,
                            inner_scale_pico_meter_100: CategorizeStageOutput<ScalePicoMeter100>,
                            inner_scale_nano_meter_1: CategorizeStageOutput<ScaleNanoMeter1>,
                            inner_scale_nano_meter_10: CategorizeStageOutput<ScaleNanoMeter10>,
                            inner_scale_nano_meter_100: CategorizeStageOutput<ScaleNanoMeter100>,
                            inner_scale_micro_meter_1: CategorizeStageOutput<ScaleMicroMeter1>,
                            inner_scale_micro_meter_10: CategorizeStageOutput<ScaleMicroMeter10>,
                            inner_scale_micro_meter_100: CategorizeStageOutput<ScaleMicroMeter100>,
                            inner_scale_milli_meter_1: CategorizeStageOutput<ScaleMilliMeter1>,
                            inner_scale_milli_meter_10: CategorizeStageOutput<ScaleMilliMeter10>,
                            inner_scale_milli_meter_100: CategorizeStageOutput<ScaleMilliMeter100>,
                            inner_scale_meter_1: CategorizeStageOutput<ScaleMeter1>,
                            inner_scale_meter_10: CategorizeStageOutput<ScaleMeter10>,
                            inner_scale_meter_100: CategorizeStageOutput<ScaleMeter100>,
                            inner_scale_kilo_meter_1: CategorizeStageOutput<ScaleKiloMeter1>,
                            inner_scale_kilo_meter_10: CategorizeStageOutput<ScaleKiloMeter10>,
                            inner_scale_kilo_meter_100: CategorizeStageOutput<ScaleKiloMeter100>,
                            inner_scale_mega_meter_1: CategorizeStageOutput<ScaleMegaMeter1>,
                            inner_scale_mega_meter_10: CategorizeStageOutput<ScaleMegaMeter10>,
                            inner_scale_mega_meter_100: CategorizeStageOutput<ScaleMegaMeter100>,
                            inner_scale_giga_meter_1: CategorizeStageOutput<ScaleGigaMeter1>,
                            inner_scale_giga_meter_10: CategorizeStageOutput<ScaleGigaMeter10>,
                            inner_scale_giga_meter_100: CategorizeStageOutput<ScaleGigaMeter100>,
                            inner_scale_tera_meter_1: CategorizeStageOutput<ScaleTeraMeter1>,
                            inner_scale_tera_meter_10: CategorizeStageOutput<ScaleTeraMeter10>,
                            inner_scale_tera_meter_100: CategorizeStageOutput<ScaleTeraMeter100>,
                            inner_scale_peta_meter_1: CategorizeStageOutput<ScalePetaMeter1>,
                            inner_scale_peta_meter_10: CategorizeStageOutput<ScalePetaMeter10>,
                            inner_scale_peta_meter_100: CategorizeStageOutput<ScalePetaMeter100>,
                            inner_scale_exa_meter_1: CategorizeStageOutput<ScaleExaMeter1>,
                            inner_scale_exa_meter_10: CategorizeStageOutput<ScaleExaMeter10>,
                            inner_scale_exa_meter_100: CategorizeStageOutput<ScaleExaMeter100>,
                            inner_scale_zetta_meter_1: CategorizeStageOutput<ScaleZettaMeter1>,
                            inner_scale_zetta_meter_10: CategorizeStageOutput<ScaleZettaMeter10>,
                            inner_scale_zetta_meter_100: CategorizeStageOutput<ScaleZettaMeter100>,
                            inner_scale_yotta_meter_1: CategorizeStageOutput<ScaleYottaMeter1>,
                            inner_scale_yotta_meter_10: CategorizeStageOutput<ScaleYottaMeter10>,
                            inner_scale_yotta_meter_100: CategorizeStageOutput<ScaleYottaMeter100>,
                            inner_scale_ronna_meter_1: CategorizeStageOutput<ScaleRonnaMeter1>,
                            inner_scale_ronna_meter_10: CategorizeStageOutput<ScaleRonnaMeter10>,
                            inner_scale_ronna_meter_100: CategorizeStageOutput<ScaleRonnaMeter100>,
                            inner_scale_quetta_meter_1: CategorizeStageOutput<ScaleQuettaMeter1>,
                            inner_scale_quetta_meter_10: CategorizeStageOutput<ScaleQuettaMeter10>,
                            inner_scale_quetta_meter_100: CategorizeStageOutput<ScaleQuettaMeter100>,
                            inner_scale_quetta_meter_1000: CategorizeStageOutput<ScaleQuettaMeter1000>,
                            inner_scale_quetta_meter_10000: CategorizeStageOutput<ScaleQuettaMeter10000>,
                            inner_scale_quetta_meter_100000: CategorizeStageOutput<ScaleQuettaMeter100000>,
                        }
                    ],
                    core_functions: [
                        fn RunEcs |main_access| -> Output {
                            let output_scale_quecto_meter_000001 = categorize_stage_run_ecs(main_access.inner_scale_quecto_meter_000001);
                            let output_scale_quecto_meter_00001 = categorize_stage_run_ecs(main_access.inner_scale_quecto_meter_00001);
                            let output_scale_quecto_meter_0001 = categorize_stage_run_ecs(main_access.inner_scale_quecto_meter_0001);
                            let output_scale_quecto_meter_001 = categorize_stage_run_ecs(main_access.inner_scale_quecto_meter_001);
                            let output_scale_quecto_meter_01 = categorize_stage_run_ecs(main_access.inner_scale_quecto_meter_01);
                            let output_scale_quecto_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_quecto_meter_1);
                            let output_scale_quecto_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_quecto_meter_10);
                            let output_scale_quecto_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_quecto_meter_100);
                            let output_scale_ronto_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_ronto_meter_1);
                            let output_scale_ronto_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_ronto_meter_10);
                            let output_scale_ronto_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_ronto_meter_100);
                            let output_scale_yocto_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_yocto_meter_1);
                            let output_scale_yocto_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_yocto_meter_10);
                            let output_scale_yocto_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_yocto_meter_100);
                            let output_scale_zepto_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_zepto_meter_1);
                            let output_scale_zepto_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_zepto_meter_10);
                            let output_scale_zepto_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_zepto_meter_100);
                            let output_scale_atto_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_atto_meter_1);
                            let output_scale_atto_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_atto_meter_10);
                            let output_scale_atto_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_atto_meter_100);
                            let output_scale_femto_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_femto_meter_1);
                            let output_scale_femto_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_femto_meter_10);
                            let output_scale_femto_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_femto_meter_100);
                            let output_scale_pico_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_pico_meter_1);
                            let output_scale_pico_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_pico_meter_10);
                            let output_scale_pico_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_pico_meter_100);
                            let output_scale_nano_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_nano_meter_1);
                            let output_scale_nano_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_nano_meter_10);
                            let output_scale_nano_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_nano_meter_100);
                            let output_scale_micro_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_micro_meter_1);
                            let output_scale_micro_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_micro_meter_10);
                            let output_scale_micro_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_micro_meter_100);
                            let output_scale_milli_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_milli_meter_1);
                            let output_scale_milli_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_milli_meter_10);
                            let output_scale_milli_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_milli_meter_100);
                            let output_scale_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_meter_1);
                            let output_scale_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_meter_10);
                            let output_scale_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_meter_100);
                            let output_scale_kilo_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_kilo_meter_1);
                            let output_scale_kilo_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_kilo_meter_10);
                            let output_scale_kilo_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_kilo_meter_100);
                            let output_scale_mega_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_mega_meter_1);
                            let output_scale_mega_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_mega_meter_10);
                            let output_scale_mega_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_mega_meter_100);
                            let output_scale_giga_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_giga_meter_1);
                            let output_scale_giga_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_giga_meter_10);
                            let output_scale_giga_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_giga_meter_100);
                            let output_scale_tera_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_tera_meter_1);
                            let output_scale_tera_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_tera_meter_10);
                            let output_scale_tera_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_tera_meter_100);
                            let output_scale_peta_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_peta_meter_1);
                            let output_scale_peta_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_peta_meter_10);
                            let output_scale_peta_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_peta_meter_100);
                            let output_scale_exa_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_exa_meter_1);
                            let output_scale_exa_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_exa_meter_10);
                            let output_scale_exa_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_exa_meter_100);
                            let output_scale_zetta_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_zetta_meter_1);
                            let output_scale_zetta_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_zetta_meter_10);
                            let output_scale_zetta_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_zetta_meter_100);
                            let output_scale_yotta_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_yotta_meter_1);
                            let output_scale_yotta_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_yotta_meter_10);
                            let output_scale_yotta_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_yotta_meter_100);
                            let output_scale_ronna_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_ronna_meter_1);
                            let output_scale_ronna_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_ronna_meter_10);
                            let output_scale_ronna_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_ronna_meter_100);
                            let output_scale_quetta_meter_1 = categorize_stage_run_ecs(main_access.inner_scale_quetta_meter_1);
                            let output_scale_quetta_meter_10 = categorize_stage_run_ecs(main_access.inner_scale_quetta_meter_10);
                            let output_scale_quetta_meter_100 = categorize_stage_run_ecs(main_access.inner_scale_quetta_meter_100);
                            let output_scale_quetta_meter_1000 = categorize_stage_run_ecs(main_access.inner_scale_quetta_meter_1000);
                            let output_scale_quetta_meter_10000 = categorize_stage_run_ecs(main_access.inner_scale_quetta_meter_10000);
                            let output_scale_quetta_meter_100000 = categorize_stage_run_ecs(main_access.inner_scale_quetta_meter_100000);

                            Output {
                                inner_scale_quecto_meter_000001: output_scale_quecto_meter_000001,
                                inner_scale_quecto_meter_00001: output_scale_quecto_meter_00001,
                                inner_scale_quecto_meter_0001: output_scale_quecto_meter_0001,
                                inner_scale_quecto_meter_001: output_scale_quecto_meter_001,
                                inner_scale_quecto_meter_01: output_scale_quecto_meter_01,
                                inner_scale_quecto_meter_1: output_scale_quecto_meter_1,
                                inner_scale_quecto_meter_10: output_scale_quecto_meter_10,
                                inner_scale_quecto_meter_100: output_scale_quecto_meter_100,
                                inner_scale_ronto_meter_1: output_scale_ronto_meter_1,
                                inner_scale_ronto_meter_10: output_scale_ronto_meter_10,
                                inner_scale_ronto_meter_100: output_scale_ronto_meter_100,
                                inner_scale_yocto_meter_1: output_scale_yocto_meter_1,
                                inner_scale_yocto_meter_10: output_scale_yocto_meter_10,
                                inner_scale_yocto_meter_100: output_scale_yocto_meter_100,
                                inner_scale_zepto_meter_1: output_scale_zepto_meter_1,
                                inner_scale_zepto_meter_10: output_scale_zepto_meter_10,
                                inner_scale_zepto_meter_100: output_scale_zepto_meter_100,
                                inner_scale_atto_meter_1: output_scale_atto_meter_1,
                                inner_scale_atto_meter_10: output_scale_atto_meter_10,
                                inner_scale_atto_meter_100: output_scale_atto_meter_100,
                                inner_scale_femto_meter_1: output_scale_femto_meter_1,
                                inner_scale_femto_meter_10: output_scale_femto_meter_10,
                                inner_scale_femto_meter_100: output_scale_femto_meter_100,
                                inner_scale_pico_meter_1: output_scale_pico_meter_1,
                                inner_scale_pico_meter_10: output_scale_pico_meter_10,
                                inner_scale_pico_meter_100: output_scale_pico_meter_100,
                                inner_scale_nano_meter_1: output_scale_nano_meter_1,
                                inner_scale_nano_meter_10: output_scale_nano_meter_10,
                                inner_scale_nano_meter_100: output_scale_nano_meter_100,
                                inner_scale_micro_meter_1: output_scale_micro_meter_1,
                                inner_scale_micro_meter_10: output_scale_micro_meter_10,
                                inner_scale_micro_meter_100: output_scale_micro_meter_100,
                                inner_scale_milli_meter_1: output_scale_milli_meter_1,
                                inner_scale_milli_meter_10: output_scale_milli_meter_10,
                                inner_scale_milli_meter_100: output_scale_milli_meter_100,
                                inner_scale_meter_1: output_scale_meter_1,
                                inner_scale_meter_10: output_scale_meter_10,
                                inner_scale_meter_100: output_scale_meter_100,
                                inner_scale_kilo_meter_1: output_scale_kilo_meter_1,
                                inner_scale_kilo_meter_10: output_scale_kilo_meter_10,
                                inner_scale_kilo_meter_100: output_scale_kilo_meter_100,
                                inner_scale_mega_meter_1: output_scale_mega_meter_1,
                                inner_scale_mega_meter_10: output_scale_mega_meter_10,
                                inner_scale_mega_meter_100: output_scale_mega_meter_100,
                                inner_scale_giga_meter_1: output_scale_giga_meter_1,
                                inner_scale_giga_meter_10: output_scale_giga_meter_10,
                                inner_scale_giga_meter_100: output_scale_giga_meter_100,
                                inner_scale_tera_meter_1: output_scale_tera_meter_1,
                                inner_scale_tera_meter_10: output_scale_tera_meter_10,
                                inner_scale_tera_meter_100: output_scale_tera_meter_100,
                                inner_scale_peta_meter_1: output_scale_peta_meter_1,
                                inner_scale_peta_meter_10: output_scale_peta_meter_10,
                                inner_scale_peta_meter_100: output_scale_peta_meter_100,
                                inner_scale_exa_meter_1: output_scale_exa_meter_1,
                                inner_scale_exa_meter_10: output_scale_exa_meter_10,
                                inner_scale_exa_meter_100: output_scale_exa_meter_100,
                                inner_scale_zetta_meter_1: output_scale_zetta_meter_1,
                                inner_scale_zetta_meter_10: output_scale_zetta_meter_10,
                                inner_scale_zetta_meter_100: output_scale_zetta_meter_100,
                                inner_scale_yotta_meter_1: output_scale_yotta_meter_1,
                                inner_scale_yotta_meter_10: output_scale_yotta_meter_10,
                                inner_scale_yotta_meter_100: output_scale_yotta_meter_100,
                                inner_scale_ronna_meter_1: output_scale_ronna_meter_1,
                                inner_scale_ronna_meter_10: output_scale_ronna_meter_10,
                                inner_scale_ronna_meter_100: output_scale_ronna_meter_100,
                                inner_scale_quetta_meter_1: output_scale_quetta_meter_1,
                                inner_scale_quetta_meter_10: output_scale_quetta_meter_10,
                                inner_scale_quetta_meter_100: output_scale_quetta_meter_100,
                                inner_scale_quetta_meter_1000: output_scale_quetta_meter_1000,
                                inner_scale_quetta_meter_10000: output_scale_quetta_meter_10000,
                                inner_scale_quetta_meter_100000: output_scale_quetta_meter_100000,
                            }
                        }
                    ]
                }
            ]
        }

        OnRemoveChunkLoader, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use bevy::prelude::ResMut;
                
                use crate::chunk_loader::workflows::external::on_remove_chunk_loader::{
                    MainAccess as ExtractUnloadChunkInputsStageMainAccess,
                    Input as ExtractUnloadChunkInputsStageInput,
                    Output as ExtractUnloadChunkInputsStageOutput,
                    run_ecs as extract_unload_chunk_input_stage_run_ecs
                };
                use crate::usf::scale::*;
            },
            user_items: {},
            stages: [
                ExtractUnloadChunkInputs: Ecs, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            inner_scale_quecto_meter_000001: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleQuectoMeter000001>,
                            inner_scale_quecto_meter_00001: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleQuectoMeter00001>,
                            inner_scale_quecto_meter_0001: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleQuectoMeter0001>,
                            inner_scale_quecto_meter_001: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleQuectoMeter001>,
                            inner_scale_quecto_meter_01: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleQuectoMeter01>,
                            inner_scale_quecto_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleQuectoMeter1>,
                            inner_scale_quecto_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleQuectoMeter10>,
                            inner_scale_quecto_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleQuectoMeter100>,
                            inner_scale_ronto_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleRontoMeter1>,
                            inner_scale_ronto_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleRontoMeter10>,
                            inner_scale_ronto_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleRontoMeter100>,
                            inner_scale_yocto_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleYoctoMeter1>,
                            inner_scale_yocto_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleYoctoMeter10>,
                            inner_scale_yocto_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleYoctoMeter100>,
                            inner_scale_zepto_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleZeptoMeter1>,
                            inner_scale_zepto_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleZeptoMeter10>,
                            inner_scale_zepto_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleZeptoMeter100>,
                            inner_scale_atto_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleAttoMeter1>,
                            inner_scale_atto_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleAttoMeter10>,
                            inner_scale_atto_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleAttoMeter100>,
                            inner_scale_femto_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleFemtoMeter1>,
                            inner_scale_femto_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleFemtoMeter10>,
                            inner_scale_femto_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleFemtoMeter100>,
                            inner_scale_pico_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScalePicoMeter1>,
                            inner_scale_pico_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScalePicoMeter10>,
                            inner_scale_pico_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScalePicoMeter100>,
                            inner_scale_nano_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleNanoMeter1>,
                            inner_scale_nano_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleNanoMeter10>,
                            inner_scale_nano_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleNanoMeter100>,
                            inner_scale_micro_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleMicroMeter1>,
                            inner_scale_micro_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleMicroMeter10>,
                            inner_scale_micro_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleMicroMeter100>,
                            inner_scale_milli_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleMilliMeter1>,
                            inner_scale_milli_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleMilliMeter10>,
                            inner_scale_milli_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleMilliMeter100>,
                            inner_scale_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleMeter1>,
                            inner_scale_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleMeter10>,
                            inner_scale_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleMeter100>,
                            inner_scale_kilo_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleKiloMeter1>,
                            inner_scale_kilo_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleKiloMeter10>,
                            inner_scale_kilo_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleKiloMeter100>,
                            inner_scale_mega_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleMegaMeter1>,
                            inner_scale_mega_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleMegaMeter10>,
                            inner_scale_mega_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleMegaMeter100>,
                            inner_scale_giga_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleGigaMeter1>,
                            inner_scale_giga_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleGigaMeter10>,
                            inner_scale_giga_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleGigaMeter100>,
                            inner_scale_tera_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleTeraMeter1>,
                            inner_scale_tera_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleTeraMeter10>,
                            inner_scale_tera_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleTeraMeter100>,
                            inner_scale_peta_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScalePetaMeter1>,
                            inner_scale_peta_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScalePetaMeter10>,
                            inner_scale_peta_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScalePetaMeter100>,
                            inner_scale_exa_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleExaMeter1>,
                            inner_scale_exa_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleExaMeter10>,
                            inner_scale_exa_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleExaMeter100>,
                            inner_scale_zetta_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleZettaMeter1>,
                            inner_scale_zetta_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleZettaMeter10>,
                            inner_scale_zetta_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleZettaMeter100>,
                            inner_scale_yotta_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleYottaMeter1>,
                            inner_scale_yotta_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleYottaMeter10>,
                            inner_scale_yotta_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleYottaMeter100>,
                            inner_scale_ronna_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleRonnaMeter1>,
                            inner_scale_ronna_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleRonnaMeter10>,
                            inner_scale_ronna_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleRonnaMeter100>,
                            inner_scale_quetta_meter_1: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleQuettaMeter1>,
                            inner_scale_quetta_meter_10: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleQuettaMeter10>,
                            inner_scale_quetta_meter_100: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleQuettaMeter100>,
                            inner_scale_quetta_meter_1000: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleQuettaMeter1000>,
                            inner_scale_quetta_meter_10000: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleQuettaMeter10000>,
                            inner_scale_quetta_meter_100000: ExtractUnloadChunkInputsStageMainAccess<'w, 's, ScaleQuettaMeter100000>,
                        }
                        struct Input {
                            inner_scale_quecto_meter_000001: ExtractUnloadChunkInputsStageInput<ScaleQuectoMeter000001>,
                            inner_scale_quecto_meter_00001: ExtractUnloadChunkInputsStageInput<ScaleQuectoMeter00001>,
                            inner_scale_quecto_meter_0001: ExtractUnloadChunkInputsStageInput<ScaleQuectoMeter0001>,
                            inner_scale_quecto_meter_001: ExtractUnloadChunkInputsStageInput<ScaleQuectoMeter001>,
                            inner_scale_quecto_meter_01: ExtractUnloadChunkInputsStageInput<ScaleQuectoMeter01>,
                            inner_scale_quecto_meter_1: ExtractUnloadChunkInputsStageInput<ScaleQuectoMeter1>,
                            inner_scale_quecto_meter_10: ExtractUnloadChunkInputsStageInput<ScaleQuectoMeter10>,
                            inner_scale_quecto_meter_100: ExtractUnloadChunkInputsStageInput<ScaleQuectoMeter100>,
                            inner_scale_ronto_meter_1: ExtractUnloadChunkInputsStageInput<ScaleRontoMeter1>,
                            inner_scale_ronto_meter_10: ExtractUnloadChunkInputsStageInput<ScaleRontoMeter10>,
                            inner_scale_ronto_meter_100: ExtractUnloadChunkInputsStageInput<ScaleRontoMeter100>,
                            inner_scale_yocto_meter_1: ExtractUnloadChunkInputsStageInput<ScaleYoctoMeter1>,
                            inner_scale_yocto_meter_10: ExtractUnloadChunkInputsStageInput<ScaleYoctoMeter10>,
                            inner_scale_yocto_meter_100: ExtractUnloadChunkInputsStageInput<ScaleYoctoMeter100>,
                            inner_scale_zepto_meter_1: ExtractUnloadChunkInputsStageInput<ScaleZeptoMeter1>,
                            inner_scale_zepto_meter_10: ExtractUnloadChunkInputsStageInput<ScaleZeptoMeter10>,
                            inner_scale_zepto_meter_100: ExtractUnloadChunkInputsStageInput<ScaleZeptoMeter100>,
                            inner_scale_atto_meter_1: ExtractUnloadChunkInputsStageInput<ScaleAttoMeter1>,
                            inner_scale_atto_meter_10: ExtractUnloadChunkInputsStageInput<ScaleAttoMeter10>,
                            inner_scale_atto_meter_100: ExtractUnloadChunkInputsStageInput<ScaleAttoMeter100>,
                            inner_scale_femto_meter_1: ExtractUnloadChunkInputsStageInput<ScaleFemtoMeter1>,
                            inner_scale_femto_meter_10: ExtractUnloadChunkInputsStageInput<ScaleFemtoMeter10>,
                            inner_scale_femto_meter_100: ExtractUnloadChunkInputsStageInput<ScaleFemtoMeter100>,
                            inner_scale_pico_meter_1: ExtractUnloadChunkInputsStageInput<ScalePicoMeter1>,
                            inner_scale_pico_meter_10: ExtractUnloadChunkInputsStageInput<ScalePicoMeter10>,
                            inner_scale_pico_meter_100: ExtractUnloadChunkInputsStageInput<ScalePicoMeter100>,
                            inner_scale_nano_meter_1: ExtractUnloadChunkInputsStageInput<ScaleNanoMeter1>,
                            inner_scale_nano_meter_10: ExtractUnloadChunkInputsStageInput<ScaleNanoMeter10>,
                            inner_scale_nano_meter_100: ExtractUnloadChunkInputsStageInput<ScaleNanoMeter100>,
                            inner_scale_micro_meter_1: ExtractUnloadChunkInputsStageInput<ScaleMicroMeter1>,
                            inner_scale_micro_meter_10: ExtractUnloadChunkInputsStageInput<ScaleMicroMeter10>,
                            inner_scale_micro_meter_100: ExtractUnloadChunkInputsStageInput<ScaleMicroMeter100>,
                            inner_scale_milli_meter_1: ExtractUnloadChunkInputsStageInput<ScaleMilliMeter1>,
                            inner_scale_milli_meter_10: ExtractUnloadChunkInputsStageInput<ScaleMilliMeter10>,
                            inner_scale_milli_meter_100: ExtractUnloadChunkInputsStageInput<ScaleMilliMeter100>,
                            inner_scale_meter_1: ExtractUnloadChunkInputsStageInput<ScaleMeter1>,
                            inner_scale_meter_10: ExtractUnloadChunkInputsStageInput<ScaleMeter10>,
                            inner_scale_meter_100: ExtractUnloadChunkInputsStageInput<ScaleMeter100>,
                            inner_scale_kilo_meter_1: ExtractUnloadChunkInputsStageInput<ScaleKiloMeter1>,
                            inner_scale_kilo_meter_10: ExtractUnloadChunkInputsStageInput<ScaleKiloMeter10>,
                            inner_scale_kilo_meter_100: ExtractUnloadChunkInputsStageInput<ScaleKiloMeter100>,
                            inner_scale_mega_meter_1: ExtractUnloadChunkInputsStageInput<ScaleMegaMeter1>,
                            inner_scale_mega_meter_10: ExtractUnloadChunkInputsStageInput<ScaleMegaMeter10>,
                            inner_scale_mega_meter_100: ExtractUnloadChunkInputsStageInput<ScaleMegaMeter100>,
                            inner_scale_giga_meter_1: ExtractUnloadChunkInputsStageInput<ScaleGigaMeter1>,
                            inner_scale_giga_meter_10: ExtractUnloadChunkInputsStageInput<ScaleGigaMeter10>,
                            inner_scale_giga_meter_100: ExtractUnloadChunkInputsStageInput<ScaleGigaMeter100>,
                            inner_scale_tera_meter_1: ExtractUnloadChunkInputsStageInput<ScaleTeraMeter1>,
                            inner_scale_tera_meter_10: ExtractUnloadChunkInputsStageInput<ScaleTeraMeter10>,
                            inner_scale_tera_meter_100: ExtractUnloadChunkInputsStageInput<ScaleTeraMeter100>,
                            inner_scale_peta_meter_1: ExtractUnloadChunkInputsStageInput<ScalePetaMeter1>,
                            inner_scale_peta_meter_10: ExtractUnloadChunkInputsStageInput<ScalePetaMeter10>,
                            inner_scale_peta_meter_100: ExtractUnloadChunkInputsStageInput<ScalePetaMeter100>,
                            inner_scale_exa_meter_1: ExtractUnloadChunkInputsStageInput<ScaleExaMeter1>,
                            inner_scale_exa_meter_10: ExtractUnloadChunkInputsStageInput<ScaleExaMeter10>,
                            inner_scale_exa_meter_100: ExtractUnloadChunkInputsStageInput<ScaleExaMeter100>,
                            inner_scale_zetta_meter_1: ExtractUnloadChunkInputsStageInput<ScaleZettaMeter1>,
                            inner_scale_zetta_meter_10: ExtractUnloadChunkInputsStageInput<ScaleZettaMeter10>,
                            inner_scale_zetta_meter_100: ExtractUnloadChunkInputsStageInput<ScaleZettaMeter100>,
                            inner_scale_yotta_meter_1: ExtractUnloadChunkInputsStageInput<ScaleYottaMeter1>,
                            inner_scale_yotta_meter_10: ExtractUnloadChunkInputsStageInput<ScaleYottaMeter10>,
                            inner_scale_yotta_meter_100: ExtractUnloadChunkInputsStageInput<ScaleYottaMeter100>,
                            inner_scale_ronna_meter_1: ExtractUnloadChunkInputsStageInput<ScaleRonnaMeter1>,
                            inner_scale_ronna_meter_10: ExtractUnloadChunkInputsStageInput<ScaleRonnaMeter10>,
                            inner_scale_ronna_meter_100: ExtractUnloadChunkInputsStageInput<ScaleRonnaMeter100>,
                            inner_scale_quetta_meter_1: ExtractUnloadChunkInputsStageInput<ScaleQuettaMeter1>,
                            inner_scale_quetta_meter_10: ExtractUnloadChunkInputsStageInput<ScaleQuettaMeter10>,
                            inner_scale_quetta_meter_100: ExtractUnloadChunkInputsStageInput<ScaleQuettaMeter100>,
                            inner_scale_quetta_meter_1000: ExtractUnloadChunkInputsStageInput<ScaleQuettaMeter1000>,
                            inner_scale_quetta_meter_10000: ExtractUnloadChunkInputsStageInput<ScaleQuettaMeter10000>,
                            inner_scale_quetta_meter_100000: ExtractUnloadChunkInputsStageInput<ScaleQuettaMeter100000>,
                        }
                        struct Output {
                            inner_scale_quecto_meter_000001: ExtractUnloadChunkInputsStageOutput<ScaleQuectoMeter000001>,
                            inner_scale_quecto_meter_00001: ExtractUnloadChunkInputsStageOutput<ScaleQuectoMeter00001>,
                            inner_scale_quecto_meter_0001: ExtractUnloadChunkInputsStageOutput<ScaleQuectoMeter0001>,
                            inner_scale_quecto_meter_001: ExtractUnloadChunkInputsStageOutput<ScaleQuectoMeter001>,
                            inner_scale_quecto_meter_01: ExtractUnloadChunkInputsStageOutput<ScaleQuectoMeter01>,
                            inner_scale_quecto_meter_1: ExtractUnloadChunkInputsStageOutput<ScaleQuectoMeter1>,
                            inner_scale_quecto_meter_10: ExtractUnloadChunkInputsStageOutput<ScaleQuectoMeter10>,
                            inner_scale_quecto_meter_100: ExtractUnloadChunkInputsStageOutput<ScaleQuectoMeter100>,
                            inner_scale_ronto_meter_1: ExtractUnloadChunkInputsStageOutput<ScaleRontoMeter1>,
                            inner_scale_ronto_meter_10: ExtractUnloadChunkInputsStageOutput<ScaleRontoMeter10>,
                            inner_scale_ronto_meter_100: ExtractUnloadChunkInputsStageOutput<ScaleRontoMeter100>,
                            inner_scale_yocto_meter_1: ExtractUnloadChunkInputsStageOutput<ScaleYoctoMeter1>,
                            inner_scale_yocto_meter_10: ExtractUnloadChunkInputsStageOutput<ScaleYoctoMeter10>,
                            inner_scale_yocto_meter_100: ExtractUnloadChunkInputsStageOutput<ScaleYoctoMeter100>,
                            inner_scale_zepto_meter_1: ExtractUnloadChunkInputsStageOutput<ScaleZeptoMeter1>,
                            inner_scale_zepto_meter_10: ExtractUnloadChunkInputsStageOutput<ScaleZeptoMeter10>,
                            inner_scale_zepto_meter_100: ExtractUnloadChunkInputsStageOutput<ScaleZeptoMeter100>,
                            inner_scale_atto_meter_1: ExtractUnloadChunkInputsStageOutput<ScaleAttoMeter1>,
                            inner_scale_atto_meter_10: ExtractUnloadChunkInputsStageOutput<ScaleAttoMeter10>,
                            inner_scale_atto_meter_100: ExtractUnloadChunkInputsStageOutput<ScaleAttoMeter100>,
                            inner_scale_femto_meter_1: ExtractUnloadChunkInputsStageOutput<ScaleFemtoMeter1>,
                            inner_scale_femto_meter_10: ExtractUnloadChunkInputsStageOutput<ScaleFemtoMeter10>,
                            inner_scale_femto_meter_100: ExtractUnloadChunkInputsStageOutput<ScaleFemtoMeter100>,
                            inner_scale_pico_meter_1: ExtractUnloadChunkInputsStageOutput<ScalePicoMeter1>,
                            inner_scale_pico_meter_10: ExtractUnloadChunkInputsStageOutput<ScalePicoMeter10>,
                            inner_scale_pico_meter_100: ExtractUnloadChunkInputsStageOutput<ScalePicoMeter100>,
                            inner_scale_nano_meter_1: ExtractUnloadChunkInputsStageOutput<ScaleNanoMeter1>,
                            inner_scale_nano_meter_10: ExtractUnloadChunkInputsStageOutput<ScaleNanoMeter10>,
                            inner_scale_nano_meter_100: ExtractUnloadChunkInputsStageOutput<ScaleNanoMeter100>,
                            inner_scale_micro_meter_1: ExtractUnloadChunkInputsStageOutput<ScaleMicroMeter1>,
                            inner_scale_micro_meter_10: ExtractUnloadChunkInputsStageOutput<ScaleMicroMeter10>,
                            inner_scale_micro_meter_100: ExtractUnloadChunkInputsStageOutput<ScaleMicroMeter100>,
                            inner_scale_milli_meter_1: ExtractUnloadChunkInputsStageOutput<ScaleMilliMeter1>,
                            inner_scale_milli_meter_10: ExtractUnloadChunkInputsStageOutput<ScaleMilliMeter10>,
                            inner_scale_milli_meter_100: ExtractUnloadChunkInputsStageOutput<ScaleMilliMeter100>,
                            inner_scale_meter_1: ExtractUnloadChunkInputsStageOutput<ScaleMeter1>,
                            inner_scale_meter_10: ExtractUnloadChunkInputsStageOutput<ScaleMeter10>,
                            inner_scale_meter_100: ExtractUnloadChunkInputsStageOutput<ScaleMeter100>,
                            inner_scale_kilo_meter_1: ExtractUnloadChunkInputsStageOutput<ScaleKiloMeter1>,
                            inner_scale_kilo_meter_10: ExtractUnloadChunkInputsStageOutput<ScaleKiloMeter10>,
                            inner_scale_kilo_meter_100: ExtractUnloadChunkInputsStageOutput<ScaleKiloMeter100>,
                            inner_scale_mega_meter_1: ExtractUnloadChunkInputsStageOutput<ScaleMegaMeter1>,
                            inner_scale_mega_meter_10: ExtractUnloadChunkInputsStageOutput<ScaleMegaMeter10>,
                            inner_scale_mega_meter_100: ExtractUnloadChunkInputsStageOutput<ScaleMegaMeter100>,
                            inner_scale_giga_meter_1: ExtractUnloadChunkInputsStageOutput<ScaleGigaMeter1>,
                            inner_scale_giga_meter_10: ExtractUnloadChunkInputsStageOutput<ScaleGigaMeter10>,
                            inner_scale_giga_meter_100: ExtractUnloadChunkInputsStageOutput<ScaleGigaMeter100>,
                            inner_scale_tera_meter_1: ExtractUnloadChunkInputsStageOutput<ScaleTeraMeter1>,
                            inner_scale_tera_meter_10: ExtractUnloadChunkInputsStageOutput<ScaleTeraMeter10>,
                            inner_scale_tera_meter_100: ExtractUnloadChunkInputsStageOutput<ScaleTeraMeter100>,
                            inner_scale_peta_meter_1: ExtractUnloadChunkInputsStageOutput<ScalePetaMeter1>,
                            inner_scale_peta_meter_10: ExtractUnloadChunkInputsStageOutput<ScalePetaMeter10>,
                            inner_scale_peta_meter_100: ExtractUnloadChunkInputsStageOutput<ScalePetaMeter100>,
                            inner_scale_exa_meter_1: ExtractUnloadChunkInputsStageOutput<ScaleExaMeter1>,
                            inner_scale_exa_meter_10: ExtractUnloadChunkInputsStageOutput<ScaleExaMeter10>,
                            inner_scale_exa_meter_100: ExtractUnloadChunkInputsStageOutput<ScaleExaMeter100>,
                            inner_scale_zetta_meter_1: ExtractUnloadChunkInputsStageOutput<ScaleZettaMeter1>,
                            inner_scale_zetta_meter_10: ExtractUnloadChunkInputsStageOutput<ScaleZettaMeter10>,
                            inner_scale_zetta_meter_100: ExtractUnloadChunkInputsStageOutput<ScaleZettaMeter100>,
                            inner_scale_yotta_meter_1: ExtractUnloadChunkInputsStageOutput<ScaleYottaMeter1>,
                            inner_scale_yotta_meter_10: ExtractUnloadChunkInputsStageOutput<ScaleYottaMeter10>,
                            inner_scale_yotta_meter_100: ExtractUnloadChunkInputsStageOutput<ScaleYottaMeter100>,
                            inner_scale_ronna_meter_1: ExtractUnloadChunkInputsStageOutput<ScaleRonnaMeter1>,
                            inner_scale_ronna_meter_10: ExtractUnloadChunkInputsStageOutput<ScaleRonnaMeter10>,
                            inner_scale_ronna_meter_100: ExtractUnloadChunkInputsStageOutput<ScaleRonnaMeter100>,
                            inner_scale_quetta_meter_1: ExtractUnloadChunkInputsStageOutput<ScaleQuettaMeter1>,
                            inner_scale_quetta_meter_10: ExtractUnloadChunkInputsStageOutput<ScaleQuettaMeter10>,
                            inner_scale_quetta_meter_100: ExtractUnloadChunkInputsStageOutput<ScaleQuettaMeter100>,
                            inner_scale_quetta_meter_1000: ExtractUnloadChunkInputsStageOutput<ScaleQuettaMeter1000>,
                            inner_scale_quetta_meter_10000: ExtractUnloadChunkInputsStageOutput<ScaleQuettaMeter10000>,
                            inner_scale_quetta_meter_100000: ExtractUnloadChunkInputsStageOutput<ScaleQuettaMeter100000>,
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| -> Output {
                            let output_scale_quecto_meter_000001 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_quecto_meter_000001, main_access.inner_scale_quecto_meter_000001);
                            let output_scale_quecto_meter_00001 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_quecto_meter_00001, main_access.inner_scale_quecto_meter_00001);
                            let output_scale_quecto_meter_0001 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_quecto_meter_0001, main_access.inner_scale_quecto_meter_0001);
                            let output_scale_quecto_meter_001 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_quecto_meter_001, main_access.inner_scale_quecto_meter_001);
                            let output_scale_quecto_meter_01 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_quecto_meter_01, main_access.inner_scale_quecto_meter_01);
                            let output_scale_quecto_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_quecto_meter_1, main_access.inner_scale_quecto_meter_1);
                            let output_scale_quecto_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_quecto_meter_10, main_access.inner_scale_quecto_meter_10);
                            let output_scale_quecto_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_quecto_meter_100, main_access.inner_scale_quecto_meter_100);
                            let output_scale_ronto_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_ronto_meter_1, main_access.inner_scale_ronto_meter_1);
                            let output_scale_ronto_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_ronto_meter_10, main_access.inner_scale_ronto_meter_10);
                            let output_scale_ronto_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_ronto_meter_100, main_access.inner_scale_ronto_meter_100);
                            let output_scale_yocto_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_yocto_meter_1, main_access.inner_scale_yocto_meter_1);
                            let output_scale_yocto_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_yocto_meter_10, main_access.inner_scale_yocto_meter_10);
                            let output_scale_yocto_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_yocto_meter_100, main_access.inner_scale_yocto_meter_100);
                            let output_scale_zepto_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_zepto_meter_1, main_access.inner_scale_zepto_meter_1);
                            let output_scale_zepto_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_zepto_meter_10, main_access.inner_scale_zepto_meter_10);
                            let output_scale_zepto_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_zepto_meter_100, main_access.inner_scale_zepto_meter_100);
                            let output_scale_atto_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_atto_meter_1, main_access.inner_scale_atto_meter_1);
                            let output_scale_atto_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_atto_meter_10, main_access.inner_scale_atto_meter_10);
                            let output_scale_atto_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_atto_meter_100, main_access.inner_scale_atto_meter_100);
                            let output_scale_femto_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_femto_meter_1, main_access.inner_scale_femto_meter_1);
                            let output_scale_femto_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_femto_meter_10, main_access.inner_scale_femto_meter_10);
                            let output_scale_femto_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_femto_meter_100, main_access.inner_scale_femto_meter_100);
                            let output_scale_pico_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_pico_meter_1, main_access.inner_scale_pico_meter_1);
                            let output_scale_pico_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_pico_meter_10, main_access.inner_scale_pico_meter_10);
                            let output_scale_pico_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_pico_meter_100, main_access.inner_scale_pico_meter_100);
                            let output_scale_nano_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_nano_meter_1, main_access.inner_scale_nano_meter_1);
                            let output_scale_nano_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_nano_meter_10, main_access.inner_scale_nano_meter_10);
                            let output_scale_nano_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_nano_meter_100, main_access.inner_scale_nano_meter_100);
                            let output_scale_micro_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_micro_meter_1, main_access.inner_scale_micro_meter_1);
                            let output_scale_micro_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_micro_meter_10, main_access.inner_scale_micro_meter_10);
                            let output_scale_micro_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_micro_meter_100, main_access.inner_scale_micro_meter_100);
                            let output_scale_milli_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_milli_meter_1, main_access.inner_scale_milli_meter_1);
                            let output_scale_milli_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_milli_meter_10, main_access.inner_scale_milli_meter_10);
                            let output_scale_milli_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_milli_meter_100, main_access.inner_scale_milli_meter_100);
                            let output_scale_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_meter_1, main_access.inner_scale_meter_1);
                            let output_scale_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_meter_10, main_access.inner_scale_meter_10);
                            let output_scale_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_meter_100, main_access.inner_scale_meter_100);
                            let output_scale_kilo_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_kilo_meter_1, main_access.inner_scale_kilo_meter_1);
                            let output_scale_kilo_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_kilo_meter_10, main_access.inner_scale_kilo_meter_10);
                            let output_scale_kilo_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_kilo_meter_100, main_access.inner_scale_kilo_meter_100);
                            let output_scale_mega_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_mega_meter_1, main_access.inner_scale_mega_meter_1);
                            let output_scale_mega_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_mega_meter_10, main_access.inner_scale_mega_meter_10);
                            let output_scale_mega_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_mega_meter_100, main_access.inner_scale_mega_meter_100);
                            let output_scale_giga_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_giga_meter_1, main_access.inner_scale_giga_meter_1);
                            let output_scale_giga_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_giga_meter_10, main_access.inner_scale_giga_meter_10);
                            let output_scale_giga_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_giga_meter_100, main_access.inner_scale_giga_meter_100);
                            let output_scale_tera_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_tera_meter_1, main_access.inner_scale_tera_meter_1);
                            let output_scale_tera_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_tera_meter_10, main_access.inner_scale_tera_meter_10);
                            let output_scale_tera_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_tera_meter_100, main_access.inner_scale_tera_meter_100);
                            let output_scale_peta_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_peta_meter_1, main_access.inner_scale_peta_meter_1);
                            let output_scale_peta_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_peta_meter_10, main_access.inner_scale_peta_meter_10);
                            let output_scale_peta_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_peta_meter_100, main_access.inner_scale_peta_meter_100);
                            let output_scale_exa_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_exa_meter_1, main_access.inner_scale_exa_meter_1);
                            let output_scale_exa_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_exa_meter_10, main_access.inner_scale_exa_meter_10);
                            let output_scale_exa_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_exa_meter_100, main_access.inner_scale_exa_meter_100);
                            let output_scale_zetta_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_zetta_meter_1, main_access.inner_scale_zetta_meter_1);
                            let output_scale_zetta_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_zetta_meter_10, main_access.inner_scale_zetta_meter_10);
                            let output_scale_zetta_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_zetta_meter_100, main_access.inner_scale_zetta_meter_100);
                            let output_scale_yotta_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_yotta_meter_1, main_access.inner_scale_yotta_meter_1);
                            let output_scale_yotta_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_yotta_meter_10, main_access.inner_scale_yotta_meter_10);
                            let output_scale_yotta_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_yotta_meter_100, main_access.inner_scale_yotta_meter_100);
                            let output_scale_ronna_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_ronna_meter_1, main_access.inner_scale_ronna_meter_1);
                            let output_scale_ronna_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_ronna_meter_10, main_access.inner_scale_ronna_meter_10);
                            let output_scale_ronna_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_ronna_meter_100, main_access.inner_scale_ronna_meter_100);
                            let output_scale_quetta_meter_1 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_quetta_meter_1, main_access.inner_scale_quetta_meter_1);
                            let output_scale_quetta_meter_10 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_quetta_meter_10, main_access.inner_scale_quetta_meter_10);
                            let output_scale_quetta_meter_100 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_quetta_meter_100, main_access.inner_scale_quetta_meter_100);
                            let output_scale_quetta_meter_1000 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_quetta_meter_1000, main_access.inner_scale_quetta_meter_1000);
                            let output_scale_quetta_meter_10000 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_quetta_meter_10000, main_access.inner_scale_quetta_meter_10000);
                            let output_scale_quetta_meter_100000 = extract_unload_chunk_input_stage_run_ecs(input.inner_scale_quetta_meter_100000, main_access.inner_scale_quetta_meter_100000);

                            Output {
                                inner_scale_quecto_meter_000001: output_scale_quecto_meter_000001,
                                inner_scale_quecto_meter_00001: output_scale_quecto_meter_00001,
                                inner_scale_quecto_meter_0001: output_scale_quecto_meter_0001,
                                inner_scale_quecto_meter_001: output_scale_quecto_meter_001,
                                inner_scale_quecto_meter_01: output_scale_quecto_meter_01,
                                inner_scale_quecto_meter_1: output_scale_quecto_meter_1,
                                inner_scale_quecto_meter_10: output_scale_quecto_meter_10,
                                inner_scale_quecto_meter_100: output_scale_quecto_meter_100,
                                inner_scale_ronto_meter_1: output_scale_ronto_meter_1,
                                inner_scale_ronto_meter_10: output_scale_ronto_meter_10,
                                inner_scale_ronto_meter_100: output_scale_ronto_meter_100,
                                inner_scale_yocto_meter_1: output_scale_yocto_meter_1,
                                inner_scale_yocto_meter_10: output_scale_yocto_meter_10,
                                inner_scale_yocto_meter_100: output_scale_yocto_meter_100,
                                inner_scale_zepto_meter_1: output_scale_zepto_meter_1,
                                inner_scale_zepto_meter_10: output_scale_zepto_meter_10,
                                inner_scale_zepto_meter_100: output_scale_zepto_meter_100,
                                inner_scale_atto_meter_1: output_scale_atto_meter_1,
                                inner_scale_atto_meter_10: output_scale_atto_meter_10,
                                inner_scale_atto_meter_100: output_scale_atto_meter_100,
                                inner_scale_femto_meter_1: output_scale_femto_meter_1,
                                inner_scale_femto_meter_10: output_scale_femto_meter_10,
                                inner_scale_femto_meter_100: output_scale_femto_meter_100,
                                inner_scale_pico_meter_1: output_scale_pico_meter_1,
                                inner_scale_pico_meter_10: output_scale_pico_meter_10,
                                inner_scale_pico_meter_100: output_scale_pico_meter_100,
                                inner_scale_nano_meter_1: output_scale_nano_meter_1,
                                inner_scale_nano_meter_10: output_scale_nano_meter_10,
                                inner_scale_nano_meter_100: output_scale_nano_meter_100,
                                inner_scale_micro_meter_1: output_scale_micro_meter_1,
                                inner_scale_micro_meter_10: output_scale_micro_meter_10,
                                inner_scale_micro_meter_100: output_scale_micro_meter_100,
                                inner_scale_milli_meter_1: output_scale_milli_meter_1,
                                inner_scale_milli_meter_10: output_scale_milli_meter_10,
                                inner_scale_milli_meter_100: output_scale_milli_meter_100,
                                inner_scale_meter_1: output_scale_meter_1,
                                inner_scale_meter_10: output_scale_meter_10,
                                inner_scale_meter_100: output_scale_meter_100,
                                inner_scale_kilo_meter_1: output_scale_kilo_meter_1,
                                inner_scale_kilo_meter_10: output_scale_kilo_meter_10,
                                inner_scale_kilo_meter_100: output_scale_kilo_meter_100,
                                inner_scale_mega_meter_1: output_scale_mega_meter_1,
                                inner_scale_mega_meter_10: output_scale_mega_meter_10,
                                inner_scale_mega_meter_100: output_scale_mega_meter_100,
                                inner_scale_giga_meter_1: output_scale_giga_meter_1,
                                inner_scale_giga_meter_10: output_scale_giga_meter_10,
                                inner_scale_giga_meter_100: output_scale_giga_meter_100,
                                inner_scale_tera_meter_1: output_scale_tera_meter_1,
                                inner_scale_tera_meter_10: output_scale_tera_meter_10,
                                inner_scale_tera_meter_100: output_scale_tera_meter_100,
                                inner_scale_peta_meter_1: output_scale_peta_meter_1,
                                inner_scale_peta_meter_10: output_scale_peta_meter_10,
                                inner_scale_peta_meter_100: output_scale_peta_meter_100,
                                inner_scale_exa_meter_1: output_scale_exa_meter_1,
                                inner_scale_exa_meter_10: output_scale_exa_meter_10,
                                inner_scale_exa_meter_100: output_scale_exa_meter_100,
                                inner_scale_zetta_meter_1: output_scale_zetta_meter_1,
                                inner_scale_zetta_meter_10: output_scale_zetta_meter_10,
                                inner_scale_zetta_meter_100: output_scale_zetta_meter_100,
                                inner_scale_yotta_meter_1: output_scale_yotta_meter_1,
                                inner_scale_yotta_meter_10: output_scale_yotta_meter_10,
                                inner_scale_yotta_meter_100: output_scale_yotta_meter_100,
                                inner_scale_ronna_meter_1: output_scale_ronna_meter_1,
                                inner_scale_ronna_meter_10: output_scale_ronna_meter_10,
                                inner_scale_ronna_meter_100: output_scale_ronna_meter_100,
                                inner_scale_quetta_meter_1: output_scale_quetta_meter_1,
                                inner_scale_quetta_meter_10: output_scale_quetta_meter_10,
                                inner_scale_quetta_meter_100: output_scale_quetta_meter_100,
                                inner_scale_quetta_meter_1000: output_scale_quetta_meter_1000,
                                inner_scale_quetta_meter_10000: output_scale_quetta_meter_10000,
                                inner_scale_quetta_meter_100000: output_scale_quetta_meter_100000,
                            }
                        }
                    ]
                }
            ],
        }

        OnRemovedChunkLoader, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use bevy::prelude::ResMut;
                
                use crate::chunk_loader::workflows::external::on_removed_chunk_loader::{
                    MainAccess as SendRemovedChunkLoaderEventStageMainAccess,
                    Input as SendRemovedChunkLoaderEventStageInput,
                    run_ecs as send_removed_chunk_loader_event_stage_run_ecs
                };
                use crate::usf::scale::*;
            },
            user_items: {},
            stages: [
                SendRemovedChunkLoaderEvent: Ecs, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            inner_scale_quecto_meter_000001: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleQuectoMeter000001>,
                            inner_scale_quecto_meter_00001: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleQuectoMeter00001>,
                            inner_scale_quecto_meter_0001: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleQuectoMeter0001>,
                            inner_scale_quecto_meter_001: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleQuectoMeter001>,
                            inner_scale_quecto_meter_01: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleQuectoMeter01>,
                            inner_scale_quecto_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleQuectoMeter1>,
                            inner_scale_quecto_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleQuectoMeter10>,
                            inner_scale_quecto_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleQuectoMeter100>,
                            inner_scale_ronto_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleRontoMeter1>,
                            inner_scale_ronto_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleRontoMeter10>,
                            inner_scale_ronto_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleRontoMeter100>,
                            inner_scale_yocto_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleYoctoMeter1>,
                            inner_scale_yocto_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleYoctoMeter10>,
                            inner_scale_yocto_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleYoctoMeter100>,
                            inner_scale_zepto_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleZeptoMeter1>,
                            inner_scale_zepto_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleZeptoMeter10>,
                            inner_scale_zepto_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleZeptoMeter100>,
                            inner_scale_atto_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleAttoMeter1>,
                            inner_scale_atto_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleAttoMeter10>,
                            inner_scale_atto_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleAttoMeter100>,
                            inner_scale_femto_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleFemtoMeter1>,
                            inner_scale_femto_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleFemtoMeter10>,
                            inner_scale_femto_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleFemtoMeter100>,
                            inner_scale_pico_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScalePicoMeter1>,
                            inner_scale_pico_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScalePicoMeter10>,
                            inner_scale_pico_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScalePicoMeter100>,
                            inner_scale_nano_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleNanoMeter1>,
                            inner_scale_nano_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleNanoMeter10>,
                            inner_scale_nano_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleNanoMeter100>,
                            inner_scale_micro_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleMicroMeter1>,
                            inner_scale_micro_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleMicroMeter10>,
                            inner_scale_micro_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleMicroMeter100>,
                            inner_scale_milli_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleMilliMeter1>,
                            inner_scale_milli_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleMilliMeter10>,
                            inner_scale_milli_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleMilliMeter100>,
                            inner_scale_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleMeter1>,
                            inner_scale_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleMeter10>,
                            inner_scale_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleMeter100>,
                            inner_scale_kilo_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleKiloMeter1>,
                            inner_scale_kilo_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleKiloMeter10>,
                            inner_scale_kilo_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleKiloMeter100>,
                            inner_scale_mega_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleMegaMeter1>,
                            inner_scale_mega_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleMegaMeter10>,
                            inner_scale_mega_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleMegaMeter100>,
                            inner_scale_giga_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleGigaMeter1>,
                            inner_scale_giga_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleGigaMeter10>,
                            inner_scale_giga_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleGigaMeter100>,
                            inner_scale_tera_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleTeraMeter1>,
                            inner_scale_tera_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleTeraMeter10>,
                            inner_scale_tera_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleTeraMeter100>,
                            inner_scale_peta_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScalePetaMeter1>,
                            inner_scale_peta_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScalePetaMeter10>,
                            inner_scale_peta_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScalePetaMeter100>,
                            inner_scale_exa_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleExaMeter1>,
                            inner_scale_exa_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleExaMeter10>,
                            inner_scale_exa_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleExaMeter100>,
                            inner_scale_zetta_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleZettaMeter1>,
                            inner_scale_zetta_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleZettaMeter10>,
                            inner_scale_zetta_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleZettaMeter100>,
                            inner_scale_yotta_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleYottaMeter1>,
                            inner_scale_yotta_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleYottaMeter10>,
                            inner_scale_yotta_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleYottaMeter100>,
                            inner_scale_ronna_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleRonnaMeter1>,
                            inner_scale_ronna_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleRonnaMeter10>,
                            inner_scale_ronna_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleRonnaMeter100>,
                            inner_scale_quetta_meter_1: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleQuettaMeter1>,
                            inner_scale_quetta_meter_10: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleQuettaMeter10>,
                            inner_scale_quetta_meter_100: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleQuettaMeter100>,
                            inner_scale_quetta_meter_1000: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleQuettaMeter1000>,
                            inner_scale_quetta_meter_10000: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleQuettaMeter10000>,
                            inner_scale_quetta_meter_100000: SendRemovedChunkLoaderEventStageMainAccess<'w, 's, ScaleQuettaMeter100000>,
                        }
                        struct Input {
                            inner_scale_quecto_meter_000001: SendRemovedChunkLoaderEventStageInput<ScaleQuectoMeter000001>,
                            inner_scale_quecto_meter_00001: SendRemovedChunkLoaderEventStageInput<ScaleQuectoMeter00001>,
                            inner_scale_quecto_meter_0001: SendRemovedChunkLoaderEventStageInput<ScaleQuectoMeter0001>,
                            inner_scale_quecto_meter_001: SendRemovedChunkLoaderEventStageInput<ScaleQuectoMeter001>,
                            inner_scale_quecto_meter_01: SendRemovedChunkLoaderEventStageInput<ScaleQuectoMeter01>,
                            inner_scale_quecto_meter_1: SendRemovedChunkLoaderEventStageInput<ScaleQuectoMeter1>,
                            inner_scale_quecto_meter_10: SendRemovedChunkLoaderEventStageInput<ScaleQuectoMeter10>,
                            inner_scale_quecto_meter_100: SendRemovedChunkLoaderEventStageInput<ScaleQuectoMeter100>,
                            inner_scale_ronto_meter_1: SendRemovedChunkLoaderEventStageInput<ScaleRontoMeter1>,
                            inner_scale_ronto_meter_10: SendRemovedChunkLoaderEventStageInput<ScaleRontoMeter10>,
                            inner_scale_ronto_meter_100: SendRemovedChunkLoaderEventStageInput<ScaleRontoMeter100>,
                            inner_scale_yocto_meter_1: SendRemovedChunkLoaderEventStageInput<ScaleYoctoMeter1>,
                            inner_scale_yocto_meter_10: SendRemovedChunkLoaderEventStageInput<ScaleYoctoMeter10>,
                            inner_scale_yocto_meter_100: SendRemovedChunkLoaderEventStageInput<ScaleYoctoMeter100>,
                            inner_scale_zepto_meter_1: SendRemovedChunkLoaderEventStageInput<ScaleZeptoMeter1>,
                            inner_scale_zepto_meter_10: SendRemovedChunkLoaderEventStageInput<ScaleZeptoMeter10>,
                            inner_scale_zepto_meter_100: SendRemovedChunkLoaderEventStageInput<ScaleZeptoMeter100>,
                            inner_scale_atto_meter_1: SendRemovedChunkLoaderEventStageInput<ScaleAttoMeter1>,
                            inner_scale_atto_meter_10: SendRemovedChunkLoaderEventStageInput<ScaleAttoMeter10>,
                            inner_scale_atto_meter_100: SendRemovedChunkLoaderEventStageInput<ScaleAttoMeter100>,
                            inner_scale_femto_meter_1: SendRemovedChunkLoaderEventStageInput<ScaleFemtoMeter1>,
                            inner_scale_femto_meter_10: SendRemovedChunkLoaderEventStageInput<ScaleFemtoMeter10>,
                            inner_scale_femto_meter_100: SendRemovedChunkLoaderEventStageInput<ScaleFemtoMeter100>,
                            inner_scale_pico_meter_1: SendRemovedChunkLoaderEventStageInput<ScalePicoMeter1>,
                            inner_scale_pico_meter_10: SendRemovedChunkLoaderEventStageInput<ScalePicoMeter10>,
                            inner_scale_pico_meter_100: SendRemovedChunkLoaderEventStageInput<ScalePicoMeter100>,
                            inner_scale_nano_meter_1: SendRemovedChunkLoaderEventStageInput<ScaleNanoMeter1>,
                            inner_scale_nano_meter_10: SendRemovedChunkLoaderEventStageInput<ScaleNanoMeter10>,
                            inner_scale_nano_meter_100: SendRemovedChunkLoaderEventStageInput<ScaleNanoMeter100>,
                            inner_scale_micro_meter_1: SendRemovedChunkLoaderEventStageInput<ScaleMicroMeter1>,
                            inner_scale_micro_meter_10: SendRemovedChunkLoaderEventStageInput<ScaleMicroMeter10>,
                            inner_scale_micro_meter_100: SendRemovedChunkLoaderEventStageInput<ScaleMicroMeter100>,
                            inner_scale_milli_meter_1: SendRemovedChunkLoaderEventStageInput<ScaleMilliMeter1>,
                            inner_scale_milli_meter_10: SendRemovedChunkLoaderEventStageInput<ScaleMilliMeter10>,
                            inner_scale_milli_meter_100: SendRemovedChunkLoaderEventStageInput<ScaleMilliMeter100>,
                            inner_scale_meter_1: SendRemovedChunkLoaderEventStageInput<ScaleMeter1>,
                            inner_scale_meter_10: SendRemovedChunkLoaderEventStageInput<ScaleMeter10>,
                            inner_scale_meter_100: SendRemovedChunkLoaderEventStageInput<ScaleMeter100>,
                            inner_scale_kilo_meter_1: SendRemovedChunkLoaderEventStageInput<ScaleKiloMeter1>,
                            inner_scale_kilo_meter_10: SendRemovedChunkLoaderEventStageInput<ScaleKiloMeter10>,
                            inner_scale_kilo_meter_100: SendRemovedChunkLoaderEventStageInput<ScaleKiloMeter100>,
                            inner_scale_mega_meter_1: SendRemovedChunkLoaderEventStageInput<ScaleMegaMeter1>,
                            inner_scale_mega_meter_10: SendRemovedChunkLoaderEventStageInput<ScaleMegaMeter10>,
                            inner_scale_mega_meter_100: SendRemovedChunkLoaderEventStageInput<ScaleMegaMeter100>,
                            inner_scale_giga_meter_1: SendRemovedChunkLoaderEventStageInput<ScaleGigaMeter1>,
                            inner_scale_giga_meter_10: SendRemovedChunkLoaderEventStageInput<ScaleGigaMeter10>,
                            inner_scale_giga_meter_100: SendRemovedChunkLoaderEventStageInput<ScaleGigaMeter100>,
                            inner_scale_tera_meter_1: SendRemovedChunkLoaderEventStageInput<ScaleTeraMeter1>,
                            inner_scale_tera_meter_10: SendRemovedChunkLoaderEventStageInput<ScaleTeraMeter10>,
                            inner_scale_tera_meter_100: SendRemovedChunkLoaderEventStageInput<ScaleTeraMeter100>,
                            inner_scale_peta_meter_1: SendRemovedChunkLoaderEventStageInput<ScalePetaMeter1>,
                            inner_scale_peta_meter_10: SendRemovedChunkLoaderEventStageInput<ScalePetaMeter10>,
                            inner_scale_peta_meter_100: SendRemovedChunkLoaderEventStageInput<ScalePetaMeter100>,
                            inner_scale_exa_meter_1: SendRemovedChunkLoaderEventStageInput<ScaleExaMeter1>,
                            inner_scale_exa_meter_10: SendRemovedChunkLoaderEventStageInput<ScaleExaMeter10>,
                            inner_scale_exa_meter_100: SendRemovedChunkLoaderEventStageInput<ScaleExaMeter100>,
                            inner_scale_zetta_meter_1: SendRemovedChunkLoaderEventStageInput<ScaleZettaMeter1>,
                            inner_scale_zetta_meter_10: SendRemovedChunkLoaderEventStageInput<ScaleZettaMeter10>,
                            inner_scale_zetta_meter_100: SendRemovedChunkLoaderEventStageInput<ScaleZettaMeter100>,
                            inner_scale_yotta_meter_1: SendRemovedChunkLoaderEventStageInput<ScaleYottaMeter1>,
                            inner_scale_yotta_meter_10: SendRemovedChunkLoaderEventStageInput<ScaleYottaMeter10>,
                            inner_scale_yotta_meter_100: SendRemovedChunkLoaderEventStageInput<ScaleYottaMeter100>,
                            inner_scale_ronna_meter_1: SendRemovedChunkLoaderEventStageInput<ScaleRonnaMeter1>,
                            inner_scale_ronna_meter_10: SendRemovedChunkLoaderEventStageInput<ScaleRonnaMeter10>,
                            inner_scale_ronna_meter_100: SendRemovedChunkLoaderEventStageInput<ScaleRonnaMeter100>,
                            inner_scale_quetta_meter_1: SendRemovedChunkLoaderEventStageInput<ScaleQuettaMeter1>,
                            inner_scale_quetta_meter_10: SendRemovedChunkLoaderEventStageInput<ScaleQuettaMeter10>,
                            inner_scale_quetta_meter_100: SendRemovedChunkLoaderEventStageInput<ScaleQuettaMeter100>,
                            inner_scale_quetta_meter_1000: SendRemovedChunkLoaderEventStageInput<ScaleQuettaMeter1000>,
                            inner_scale_quetta_meter_10000: SendRemovedChunkLoaderEventStageInput<ScaleQuettaMeter10000>,
                            inner_scale_quetta_meter_100000: SendRemovedChunkLoaderEventStageInput<ScaleQuettaMeter100000>,
                        }
                    ],
                    core_functions: [
                        fn RunEcs |input, main_access| {
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_quecto_meter_000001, main_access.inner_scale_quecto_meter_000001);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_quecto_meter_00001, main_access.inner_scale_quecto_meter_00001);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_quecto_meter_0001, main_access.inner_scale_quecto_meter_0001);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_quecto_meter_001, main_access.inner_scale_quecto_meter_001);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_quecto_meter_01, main_access.inner_scale_quecto_meter_01);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_quecto_meter_1, main_access.inner_scale_quecto_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_quecto_meter_10, main_access.inner_scale_quecto_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_quecto_meter_100, main_access.inner_scale_quecto_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_ronto_meter_1, main_access.inner_scale_ronto_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_ronto_meter_10, main_access.inner_scale_ronto_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_ronto_meter_100, main_access.inner_scale_ronto_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_yocto_meter_1, main_access.inner_scale_yocto_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_yocto_meter_10, main_access.inner_scale_yocto_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_yocto_meter_100, main_access.inner_scale_yocto_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_zepto_meter_1, main_access.inner_scale_zepto_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_zepto_meter_10, main_access.inner_scale_zepto_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_zepto_meter_100, main_access.inner_scale_zepto_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_atto_meter_1, main_access.inner_scale_atto_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_atto_meter_10, main_access.inner_scale_atto_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_atto_meter_100, main_access.inner_scale_atto_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_femto_meter_1, main_access.inner_scale_femto_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_femto_meter_10, main_access.inner_scale_femto_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_femto_meter_100, main_access.inner_scale_femto_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_pico_meter_1, main_access.inner_scale_pico_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_pico_meter_10, main_access.inner_scale_pico_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_pico_meter_100, main_access.inner_scale_pico_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_nano_meter_1, main_access.inner_scale_nano_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_nano_meter_10, main_access.inner_scale_nano_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_nano_meter_100, main_access.inner_scale_nano_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_micro_meter_1, main_access.inner_scale_micro_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_micro_meter_10, main_access.inner_scale_micro_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_micro_meter_100, main_access.inner_scale_micro_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_milli_meter_1, main_access.inner_scale_milli_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_milli_meter_10, main_access.inner_scale_milli_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_milli_meter_100, main_access.inner_scale_milli_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_meter_1, main_access.inner_scale_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_meter_10, main_access.inner_scale_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_meter_100, main_access.inner_scale_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_kilo_meter_1, main_access.inner_scale_kilo_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_kilo_meter_10, main_access.inner_scale_kilo_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_kilo_meter_100, main_access.inner_scale_kilo_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_mega_meter_1, main_access.inner_scale_mega_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_mega_meter_10, main_access.inner_scale_mega_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_mega_meter_100, main_access.inner_scale_mega_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_giga_meter_1, main_access.inner_scale_giga_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_giga_meter_10, main_access.inner_scale_giga_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_giga_meter_100, main_access.inner_scale_giga_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_tera_meter_1, main_access.inner_scale_tera_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_tera_meter_10, main_access.inner_scale_tera_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_tera_meter_100, main_access.inner_scale_tera_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_peta_meter_1, main_access.inner_scale_peta_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_peta_meter_10, main_access.inner_scale_peta_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_peta_meter_100, main_access.inner_scale_peta_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_exa_meter_1, main_access.inner_scale_exa_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_exa_meter_10, main_access.inner_scale_exa_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_exa_meter_100, main_access.inner_scale_exa_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_zetta_meter_1, main_access.inner_scale_zetta_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_zetta_meter_10, main_access.inner_scale_zetta_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_zetta_meter_100, main_access.inner_scale_zetta_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_yotta_meter_1, main_access.inner_scale_yotta_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_yotta_meter_10, main_access.inner_scale_yotta_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_yotta_meter_100, main_access.inner_scale_yotta_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_ronna_meter_1, main_access.inner_scale_ronna_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_ronna_meter_10, main_access.inner_scale_ronna_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_ronna_meter_100, main_access.inner_scale_ronna_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_quetta_meter_1, main_access.inner_scale_quetta_meter_1);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_quetta_meter_10, main_access.inner_scale_quetta_meter_10);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_quetta_meter_100, main_access.inner_scale_quetta_meter_100);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_quetta_meter_1000, main_access.inner_scale_quetta_meter_1000);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_quetta_meter_10000, main_access.inner_scale_quetta_meter_10000);
                            send_removed_chunk_loader_event_stage_run_ecs(input.inner_scale_quetta_meter_100000, main_access.inner_scale_quetta_meter_100000);
                        }
                    ]
                }
            ],
        }

        LoadChunks, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use bevy::prelude::ResMut;
                
                use crate::chunk_loader::workflows::external::load_chunks::{
                    MainAccess as ValidateAndLoadAndWaitStageMainAccess,
                    Input as ValidateAndLoadAndWaitStageInput,
                    State as ValidateAndLoadAndWaitStageState,
                    setup_ecs_while as validate_and_load_and_wait_stage_setup_ecs_while,
                    run_ecs_while as validate_and_load_and_wait_stage_run_ecs_while,
                };
                use crate::usf::scale::*;
            },
            user_items: {
            },
            stages: [
                ValidateAndLoadAndWait: EcsWhile, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            inner_scale_quecto_meter_000001: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleQuectoMeter000001>,
                            inner_scale_quecto_meter_00001: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleQuectoMeter00001>,
                            inner_scale_quecto_meter_0001: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleQuectoMeter0001>,
                            inner_scale_quecto_meter_001: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleQuectoMeter001>,
                            inner_scale_quecto_meter_01: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleQuectoMeter01>,
                            inner_scale_quecto_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleQuectoMeter1>,
                            inner_scale_quecto_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleQuectoMeter10>,
                            inner_scale_quecto_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleQuectoMeter100>,
                            inner_scale_ronto_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleRontoMeter1>,
                            inner_scale_ronto_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleRontoMeter10>,
                            inner_scale_ronto_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleRontoMeter100>,
                            inner_scale_yocto_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleYoctoMeter1>,
                            inner_scale_yocto_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleYoctoMeter10>,
                            inner_scale_yocto_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleYoctoMeter100>,
                            inner_scale_zepto_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleZeptoMeter1>,
                            inner_scale_zepto_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleZeptoMeter10>,
                            inner_scale_zepto_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleZeptoMeter100>,
                            inner_scale_atto_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleAttoMeter1>,
                            inner_scale_atto_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleAttoMeter10>,
                            inner_scale_atto_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleAttoMeter100>,
                            inner_scale_femto_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleFemtoMeter1>,
                            inner_scale_femto_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleFemtoMeter10>,
                            inner_scale_femto_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleFemtoMeter100>,
                            inner_scale_pico_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScalePicoMeter1>,
                            inner_scale_pico_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScalePicoMeter10>,
                            inner_scale_pico_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScalePicoMeter100>,
                            inner_scale_nano_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleNanoMeter1>,
                            inner_scale_nano_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleNanoMeter10>,
                            inner_scale_nano_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleNanoMeter100>,
                            inner_scale_micro_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleMicroMeter1>,
                            inner_scale_micro_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleMicroMeter10>,
                            inner_scale_micro_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleMicroMeter100>,
                            inner_scale_milli_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleMilliMeter1>,
                            inner_scale_milli_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleMilliMeter10>,
                            inner_scale_milli_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleMilliMeter100>,
                            inner_scale_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleMeter1>,
                            inner_scale_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleMeter10>,
                            inner_scale_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleMeter100>,
                            inner_scale_kilo_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleKiloMeter1>,
                            inner_scale_kilo_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleKiloMeter10>,
                            inner_scale_kilo_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleKiloMeter100>,
                            inner_scale_mega_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleMegaMeter1>,
                            inner_scale_mega_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleMegaMeter10>,
                            inner_scale_mega_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleMegaMeter100>,
                            inner_scale_giga_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleGigaMeter1>,
                            inner_scale_giga_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleGigaMeter10>,
                            inner_scale_giga_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleGigaMeter100>,
                            inner_scale_tera_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleTeraMeter1>,
                            inner_scale_tera_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleTeraMeter10>,
                            inner_scale_tera_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleTeraMeter100>,
                            inner_scale_peta_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScalePetaMeter1>,
                            inner_scale_peta_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScalePetaMeter10>,
                            inner_scale_peta_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScalePetaMeter100>,
                            inner_scale_exa_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleExaMeter1>,
                            inner_scale_exa_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleExaMeter10>,
                            inner_scale_exa_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleExaMeter100>,
                            inner_scale_zetta_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleZettaMeter1>,
                            inner_scale_zetta_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleZettaMeter10>,
                            inner_scale_zetta_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleZettaMeter100>,
                            inner_scale_yotta_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleYottaMeter1>,
                            inner_scale_yotta_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleYottaMeter10>,
                            inner_scale_yotta_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleYottaMeter100>,
                            inner_scale_ronna_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleRonnaMeter1>,
                            inner_scale_ronna_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleRonnaMeter10>,
                            inner_scale_ronna_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleRonnaMeter100>,
                            inner_scale_quetta_meter_1: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleQuettaMeter1>,
                            inner_scale_quetta_meter_10: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleQuettaMeter10>,
                            inner_scale_quetta_meter_100: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleQuettaMeter100>,
                            inner_scale_quetta_meter_1000: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleQuettaMeter1000>,
                            inner_scale_quetta_meter_10000: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleQuettaMeter10000>,
                            inner_scale_quetta_meter_100000: ValidateAndLoadAndWaitStageMainAccess<'w, 's, ScaleQuettaMeter100000>,
                        }
                        struct Input {
                            inner_scale_quecto_meter_000001: ValidateAndLoadAndWaitStageInput<ScaleQuectoMeter000001>,
                            inner_scale_quecto_meter_00001: ValidateAndLoadAndWaitStageInput<ScaleQuectoMeter00001>,
                            inner_scale_quecto_meter_0001: ValidateAndLoadAndWaitStageInput<ScaleQuectoMeter0001>,
                            inner_scale_quecto_meter_001: ValidateAndLoadAndWaitStageInput<ScaleQuectoMeter001>,
                            inner_scale_quecto_meter_01: ValidateAndLoadAndWaitStageInput<ScaleQuectoMeter01>,
                            inner_scale_quecto_meter_1: ValidateAndLoadAndWaitStageInput<ScaleQuectoMeter1>,
                            inner_scale_quecto_meter_10: ValidateAndLoadAndWaitStageInput<ScaleQuectoMeter10>,
                            inner_scale_quecto_meter_100: ValidateAndLoadAndWaitStageInput<ScaleQuectoMeter100>,
                            inner_scale_ronto_meter_1: ValidateAndLoadAndWaitStageInput<ScaleRontoMeter1>,
                            inner_scale_ronto_meter_10: ValidateAndLoadAndWaitStageInput<ScaleRontoMeter10>,
                            inner_scale_ronto_meter_100: ValidateAndLoadAndWaitStageInput<ScaleRontoMeter100>,
                            inner_scale_yocto_meter_1: ValidateAndLoadAndWaitStageInput<ScaleYoctoMeter1>,
                            inner_scale_yocto_meter_10: ValidateAndLoadAndWaitStageInput<ScaleYoctoMeter10>,
                            inner_scale_yocto_meter_100: ValidateAndLoadAndWaitStageInput<ScaleYoctoMeter100>,
                            inner_scale_zepto_meter_1: ValidateAndLoadAndWaitStageInput<ScaleZeptoMeter1>,
                            inner_scale_zepto_meter_10: ValidateAndLoadAndWaitStageInput<ScaleZeptoMeter10>,
                            inner_scale_zepto_meter_100: ValidateAndLoadAndWaitStageInput<ScaleZeptoMeter100>,
                            inner_scale_atto_meter_1: ValidateAndLoadAndWaitStageInput<ScaleAttoMeter1>,
                            inner_scale_atto_meter_10: ValidateAndLoadAndWaitStageInput<ScaleAttoMeter10>,
                            inner_scale_atto_meter_100: ValidateAndLoadAndWaitStageInput<ScaleAttoMeter100>,
                            inner_scale_femto_meter_1: ValidateAndLoadAndWaitStageInput<ScaleFemtoMeter1>,
                            inner_scale_femto_meter_10: ValidateAndLoadAndWaitStageInput<ScaleFemtoMeter10>,
                            inner_scale_femto_meter_100: ValidateAndLoadAndWaitStageInput<ScaleFemtoMeter100>,
                            inner_scale_pico_meter_1: ValidateAndLoadAndWaitStageInput<ScalePicoMeter1>,
                            inner_scale_pico_meter_10: ValidateAndLoadAndWaitStageInput<ScalePicoMeter10>,
                            inner_scale_pico_meter_100: ValidateAndLoadAndWaitStageInput<ScalePicoMeter100>,
                            inner_scale_nano_meter_1: ValidateAndLoadAndWaitStageInput<ScaleNanoMeter1>,
                            inner_scale_nano_meter_10: ValidateAndLoadAndWaitStageInput<ScaleNanoMeter10>,
                            inner_scale_nano_meter_100: ValidateAndLoadAndWaitStageInput<ScaleNanoMeter100>,
                            inner_scale_micro_meter_1: ValidateAndLoadAndWaitStageInput<ScaleMicroMeter1>,
                            inner_scale_micro_meter_10: ValidateAndLoadAndWaitStageInput<ScaleMicroMeter10>,
                            inner_scale_micro_meter_100: ValidateAndLoadAndWaitStageInput<ScaleMicroMeter100>,
                            inner_scale_milli_meter_1: ValidateAndLoadAndWaitStageInput<ScaleMilliMeter1>,
                            inner_scale_milli_meter_10: ValidateAndLoadAndWaitStageInput<ScaleMilliMeter10>,
                            inner_scale_milli_meter_100: ValidateAndLoadAndWaitStageInput<ScaleMilliMeter100>,
                            inner_scale_meter_1: ValidateAndLoadAndWaitStageInput<ScaleMeter1>,
                            inner_scale_meter_10: ValidateAndLoadAndWaitStageInput<ScaleMeter10>,
                            inner_scale_meter_100: ValidateAndLoadAndWaitStageInput<ScaleMeter100>,
                            inner_scale_kilo_meter_1: ValidateAndLoadAndWaitStageInput<ScaleKiloMeter1>,
                            inner_scale_kilo_meter_10: ValidateAndLoadAndWaitStageInput<ScaleKiloMeter10>,
                            inner_scale_kilo_meter_100: ValidateAndLoadAndWaitStageInput<ScaleKiloMeter100>,
                            inner_scale_mega_meter_1: ValidateAndLoadAndWaitStageInput<ScaleMegaMeter1>,
                            inner_scale_mega_meter_10: ValidateAndLoadAndWaitStageInput<ScaleMegaMeter10>,
                            inner_scale_mega_meter_100: ValidateAndLoadAndWaitStageInput<ScaleMegaMeter100>,
                            inner_scale_giga_meter_1: ValidateAndLoadAndWaitStageInput<ScaleGigaMeter1>,
                            inner_scale_giga_meter_10: ValidateAndLoadAndWaitStageInput<ScaleGigaMeter10>,
                            inner_scale_giga_meter_100: ValidateAndLoadAndWaitStageInput<ScaleGigaMeter100>,
                            inner_scale_tera_meter_1: ValidateAndLoadAndWaitStageInput<ScaleTeraMeter1>,
                            inner_scale_tera_meter_10: ValidateAndLoadAndWaitStageInput<ScaleTeraMeter10>,
                            inner_scale_tera_meter_100: ValidateAndLoadAndWaitStageInput<ScaleTeraMeter100>,
                            inner_scale_peta_meter_1: ValidateAndLoadAndWaitStageInput<ScalePetaMeter1>,
                            inner_scale_peta_meter_10: ValidateAndLoadAndWaitStageInput<ScalePetaMeter10>,
                            inner_scale_peta_meter_100: ValidateAndLoadAndWaitStageInput<ScalePetaMeter100>,
                            inner_scale_exa_meter_1: ValidateAndLoadAndWaitStageInput<ScaleExaMeter1>,
                            inner_scale_exa_meter_10: ValidateAndLoadAndWaitStageInput<ScaleExaMeter10>,
                            inner_scale_exa_meter_100: ValidateAndLoadAndWaitStageInput<ScaleExaMeter100>,
                            inner_scale_zetta_meter_1: ValidateAndLoadAndWaitStageInput<ScaleZettaMeter1>,
                            inner_scale_zetta_meter_10: ValidateAndLoadAndWaitStageInput<ScaleZettaMeter10>,
                            inner_scale_zetta_meter_100: ValidateAndLoadAndWaitStageInput<ScaleZettaMeter100>,
                            inner_scale_yotta_meter_1: ValidateAndLoadAndWaitStageInput<ScaleYottaMeter1>,
                            inner_scale_yotta_meter_10: ValidateAndLoadAndWaitStageInput<ScaleYottaMeter10>,
                            inner_scale_yotta_meter_100: ValidateAndLoadAndWaitStageInput<ScaleYottaMeter100>,
                            inner_scale_ronna_meter_1: ValidateAndLoadAndWaitStageInput<ScaleRonnaMeter1>,
                            inner_scale_ronna_meter_10: ValidateAndLoadAndWaitStageInput<ScaleRonnaMeter10>,
                            inner_scale_ronna_meter_100: ValidateAndLoadAndWaitStageInput<ScaleRonnaMeter100>,
                            inner_scale_quetta_meter_1: ValidateAndLoadAndWaitStageInput<ScaleQuettaMeter1>,
                            inner_scale_quetta_meter_10: ValidateAndLoadAndWaitStageInput<ScaleQuettaMeter10>,
                            inner_scale_quetta_meter_100: ValidateAndLoadAndWaitStageInput<ScaleQuettaMeter100>,
                            inner_scale_quetta_meter_1000: ValidateAndLoadAndWaitStageInput<ScaleQuettaMeter1000>,
                            inner_scale_quetta_meter_10000: ValidateAndLoadAndWaitStageInput<ScaleQuettaMeter10000>,
                            inner_scale_quetta_meter_100000: ValidateAndLoadAndWaitStageInput<ScaleQuettaMeter100000>,
                        }
                        struct State {
                            inner_scale_quecto_meter_000001: Option<ValidateAndLoadAndWaitStageState<ScaleQuectoMeter000001>>,
                            inner_scale_quecto_meter_00001: Option<ValidateAndLoadAndWaitStageState<ScaleQuectoMeter00001>>,
                            inner_scale_quecto_meter_0001: Option<ValidateAndLoadAndWaitStageState<ScaleQuectoMeter0001>>,
                            inner_scale_quecto_meter_001: Option<ValidateAndLoadAndWaitStageState<ScaleQuectoMeter001>>,
                            inner_scale_quecto_meter_01: Option<ValidateAndLoadAndWaitStageState<ScaleQuectoMeter01>>,
                            inner_scale_quecto_meter_1: Option<ValidateAndLoadAndWaitStageState<ScaleQuectoMeter1>>,
                            inner_scale_quecto_meter_10: Option<ValidateAndLoadAndWaitStageState<ScaleQuectoMeter10>>,
                            inner_scale_quecto_meter_100: Option<ValidateAndLoadAndWaitStageState<ScaleQuectoMeter100>>,
                            inner_scale_ronto_meter_1: Option<ValidateAndLoadAndWaitStageState<ScaleRontoMeter1>>,
                            inner_scale_ronto_meter_10: Option<ValidateAndLoadAndWaitStageState<ScaleRontoMeter10>>,
                            inner_scale_ronto_meter_100: Option<ValidateAndLoadAndWaitStageState<ScaleRontoMeter100>>,
                            inner_scale_yocto_meter_1: Option<ValidateAndLoadAndWaitStageState<ScaleYoctoMeter1>>,
                            inner_scale_yocto_meter_10: Option<ValidateAndLoadAndWaitStageState<ScaleYoctoMeter10>>,
                            inner_scale_yocto_meter_100: Option<ValidateAndLoadAndWaitStageState<ScaleYoctoMeter100>>,
                            inner_scale_zepto_meter_1: Option<ValidateAndLoadAndWaitStageState<ScaleZeptoMeter1>>,
                            inner_scale_zepto_meter_10: Option<ValidateAndLoadAndWaitStageState<ScaleZeptoMeter10>>,
                            inner_scale_zepto_meter_100: Option<ValidateAndLoadAndWaitStageState<ScaleZeptoMeter100>>,
                            inner_scale_atto_meter_1: Option<ValidateAndLoadAndWaitStageState<ScaleAttoMeter1>>,
                            inner_scale_atto_meter_10: Option<ValidateAndLoadAndWaitStageState<ScaleAttoMeter10>>,
                            inner_scale_atto_meter_100: Option<ValidateAndLoadAndWaitStageState<ScaleAttoMeter100>>,
                            inner_scale_femto_meter_1: Option<ValidateAndLoadAndWaitStageState<ScaleFemtoMeter1>>,
                            inner_scale_femto_meter_10: Option<ValidateAndLoadAndWaitStageState<ScaleFemtoMeter10>>,
                            inner_scale_femto_meter_100: Option<ValidateAndLoadAndWaitStageState<ScaleFemtoMeter100>>,
                            inner_scale_pico_meter_1: Option<ValidateAndLoadAndWaitStageState<ScalePicoMeter1>>,
                            inner_scale_pico_meter_10: Option<ValidateAndLoadAndWaitStageState<ScalePicoMeter10>>,
                            inner_scale_pico_meter_100: Option<ValidateAndLoadAndWaitStageState<ScalePicoMeter100>>,
                            inner_scale_nano_meter_1: Option<ValidateAndLoadAndWaitStageState<ScaleNanoMeter1>>,
                            inner_scale_nano_meter_10: Option<ValidateAndLoadAndWaitStageState<ScaleNanoMeter10>>,
                            inner_scale_nano_meter_100: Option<ValidateAndLoadAndWaitStageState<ScaleNanoMeter100>>,
                            inner_scale_micro_meter_1: Option<ValidateAndLoadAndWaitStageState<ScaleMicroMeter1>>,
                            inner_scale_micro_meter_10: Option<ValidateAndLoadAndWaitStageState<ScaleMicroMeter10>>,
                            inner_scale_micro_meter_100: Option<ValidateAndLoadAndWaitStageState<ScaleMicroMeter100>>,
                            inner_scale_milli_meter_1: Option<ValidateAndLoadAndWaitStageState<ScaleMilliMeter1>>,
                            inner_scale_milli_meter_10: Option<ValidateAndLoadAndWaitStageState<ScaleMilliMeter10>>,
                            inner_scale_milli_meter_100: Option<ValidateAndLoadAndWaitStageState<ScaleMilliMeter100>>,
                            inner_scale_meter_1: Option<ValidateAndLoadAndWaitStageState<ScaleMeter1>>,
                            inner_scale_meter_10: Option<ValidateAndLoadAndWaitStageState<ScaleMeter10>>,
                            inner_scale_meter_100: Option<ValidateAndLoadAndWaitStageState<ScaleMeter100>>,
                            inner_scale_kilo_meter_1: Option<ValidateAndLoadAndWaitStageState<ScaleKiloMeter1>>,
                            inner_scale_kilo_meter_10: Option<ValidateAndLoadAndWaitStageState<ScaleKiloMeter10>>,
                            inner_scale_kilo_meter_100: Option<ValidateAndLoadAndWaitStageState<ScaleKiloMeter100>>,
                            inner_scale_mega_meter_1: Option<ValidateAndLoadAndWaitStageState<ScaleMegaMeter1>>,
                            inner_scale_mega_meter_10: Option<ValidateAndLoadAndWaitStageState<ScaleMegaMeter10>>,
                            inner_scale_mega_meter_100: Option<ValidateAndLoadAndWaitStageState<ScaleMegaMeter100>>,
                            inner_scale_giga_meter_1: Option<ValidateAndLoadAndWaitStageState<ScaleGigaMeter1>>,
                            inner_scale_giga_meter_10: Option<ValidateAndLoadAndWaitStageState<ScaleGigaMeter10>>,
                            inner_scale_giga_meter_100: Option<ValidateAndLoadAndWaitStageState<ScaleGigaMeter100>>,
                            inner_scale_tera_meter_1: Option<ValidateAndLoadAndWaitStageState<ScaleTeraMeter1>>,
                            inner_scale_tera_meter_10: Option<ValidateAndLoadAndWaitStageState<ScaleTeraMeter10>>,
                            inner_scale_tera_meter_100: Option<ValidateAndLoadAndWaitStageState<ScaleTeraMeter100>>,
                            inner_scale_peta_meter_1: Option<ValidateAndLoadAndWaitStageState<ScalePetaMeter1>>,
                            inner_scale_peta_meter_10: Option<ValidateAndLoadAndWaitStageState<ScalePetaMeter10>>,
                            inner_scale_peta_meter_100: Option<ValidateAndLoadAndWaitStageState<ScalePetaMeter100>>,
                            inner_scale_exa_meter_1: Option<ValidateAndLoadAndWaitStageState<ScaleExaMeter1>>,
                            inner_scale_exa_meter_10: Option<ValidateAndLoadAndWaitStageState<ScaleExaMeter10>>,
                            inner_scale_exa_meter_100: Option<ValidateAndLoadAndWaitStageState<ScaleExaMeter100>>,
                            inner_scale_zetta_meter_1: Option<ValidateAndLoadAndWaitStageState<ScaleZettaMeter1>>,
                            inner_scale_zetta_meter_10: Option<ValidateAndLoadAndWaitStageState<ScaleZettaMeter10>>,
                            inner_scale_zetta_meter_100: Option<ValidateAndLoadAndWaitStageState<ScaleZettaMeter100>>,
                            inner_scale_yotta_meter_1: Option<ValidateAndLoadAndWaitStageState<ScaleYottaMeter1>>,
                            inner_scale_yotta_meter_10: Option<ValidateAndLoadAndWaitStageState<ScaleYottaMeter10>>,
                            inner_scale_yotta_meter_100: Option<ValidateAndLoadAndWaitStageState<ScaleYottaMeter100>>,
                            inner_scale_ronna_meter_1: Option<ValidateAndLoadAndWaitStageState<ScaleRonnaMeter1>>,
                            inner_scale_ronna_meter_10: Option<ValidateAndLoadAndWaitStageState<ScaleRonnaMeter10>>,
                            inner_scale_ronna_meter_100: Option<ValidateAndLoadAndWaitStageState<ScaleRonnaMeter100>>,
                            inner_scale_quetta_meter_1: Option<ValidateAndLoadAndWaitStageState<ScaleQuettaMeter1>>,
                            inner_scale_quetta_meter_10: Option<ValidateAndLoadAndWaitStageState<ScaleQuettaMeter10>>,
                            inner_scale_quetta_meter_100: Option<ValidateAndLoadAndWaitStageState<ScaleQuettaMeter100>>,
                            inner_scale_quetta_meter_1000: Option<ValidateAndLoadAndWaitStageState<ScaleQuettaMeter1000>>,
                            inner_scale_quetta_meter_10000: Option<ValidateAndLoadAndWaitStageState<ScaleQuettaMeter10000>>,
                            inner_scale_quetta_meter_100000: Option<ValidateAndLoadAndWaitStageState<ScaleQuettaMeter100000>>,
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |input, main_access| -> State {
                            let state_scale_quecto_meter_000001 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_quecto_meter_000001, main_access.inner_scale_quecto_meter_000001);
                            let state_scale_quecto_meter_00001 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_quecto_meter_00001, main_access.inner_scale_quecto_meter_00001);
                            let state_scale_quecto_meter_0001 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_quecto_meter_0001, main_access.inner_scale_quecto_meter_0001);
                            let state_scale_quecto_meter_001 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_quecto_meter_001, main_access.inner_scale_quecto_meter_001);
                            let state_scale_quecto_meter_01 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_quecto_meter_01, main_access.inner_scale_quecto_meter_01);
                            let state_scale_quecto_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_quecto_meter_1, main_access.inner_scale_quecto_meter_1);
                            let state_scale_quecto_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_quecto_meter_10, main_access.inner_scale_quecto_meter_10);
                            let state_scale_quecto_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_quecto_meter_100, main_access.inner_scale_quecto_meter_100);
                            let state_scale_ronto_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_ronto_meter_1, main_access.inner_scale_ronto_meter_1);
                            let state_scale_ronto_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_ronto_meter_10, main_access.inner_scale_ronto_meter_10);
                            let state_scale_ronto_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_ronto_meter_100, main_access.inner_scale_ronto_meter_100);
                            let state_scale_yocto_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_yocto_meter_1, main_access.inner_scale_yocto_meter_1);
                            let state_scale_yocto_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_yocto_meter_10, main_access.inner_scale_yocto_meter_10);
                            let state_scale_yocto_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_yocto_meter_100, main_access.inner_scale_yocto_meter_100);
                            let state_scale_zepto_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_zepto_meter_1, main_access.inner_scale_zepto_meter_1);
                            let state_scale_zepto_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_zepto_meter_10, main_access.inner_scale_zepto_meter_10);
                            let state_scale_zepto_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_zepto_meter_100, main_access.inner_scale_zepto_meter_100);
                            let state_scale_atto_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_atto_meter_1, main_access.inner_scale_atto_meter_1);
                            let state_scale_atto_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_atto_meter_10, main_access.inner_scale_atto_meter_10);
                            let state_scale_atto_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_atto_meter_100, main_access.inner_scale_atto_meter_100);
                            let state_scale_femto_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_femto_meter_1, main_access.inner_scale_femto_meter_1);
                            let state_scale_femto_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_femto_meter_10, main_access.inner_scale_femto_meter_10);
                            let state_scale_femto_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_femto_meter_100, main_access.inner_scale_femto_meter_100);
                            let state_scale_pico_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_pico_meter_1, main_access.inner_scale_pico_meter_1);
                            let state_scale_pico_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_pico_meter_10, main_access.inner_scale_pico_meter_10);
                            let state_scale_pico_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_pico_meter_100, main_access.inner_scale_pico_meter_100);
                            let state_scale_nano_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_nano_meter_1, main_access.inner_scale_nano_meter_1);
                            let state_scale_nano_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_nano_meter_10, main_access.inner_scale_nano_meter_10);
                            let state_scale_nano_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_nano_meter_100, main_access.inner_scale_nano_meter_100);
                            let state_scale_micro_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_micro_meter_1, main_access.inner_scale_micro_meter_1);
                            let state_scale_micro_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_micro_meter_10, main_access.inner_scale_micro_meter_10);
                            let state_scale_micro_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_micro_meter_100, main_access.inner_scale_micro_meter_100);
                            let state_scale_milli_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_milli_meter_1, main_access.inner_scale_milli_meter_1);
                            let state_scale_milli_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_milli_meter_10, main_access.inner_scale_milli_meter_10);
                            let state_scale_milli_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_milli_meter_100, main_access.inner_scale_milli_meter_100);
                            let state_scale_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_meter_1, main_access.inner_scale_meter_1);
                            let state_scale_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_meter_10, main_access.inner_scale_meter_10);
                            let state_scale_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_meter_100, main_access.inner_scale_meter_100);
                            let state_scale_kilo_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_kilo_meter_1, main_access.inner_scale_kilo_meter_1);
                            let state_scale_kilo_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_kilo_meter_10, main_access.inner_scale_kilo_meter_10);
                            let state_scale_kilo_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_kilo_meter_100, main_access.inner_scale_kilo_meter_100);
                            let state_scale_mega_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_mega_meter_1, main_access.inner_scale_mega_meter_1);
                            let state_scale_mega_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_mega_meter_10, main_access.inner_scale_mega_meter_10);
                            let state_scale_mega_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_mega_meter_100, main_access.inner_scale_mega_meter_100);
                            let state_scale_giga_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_giga_meter_1, main_access.inner_scale_giga_meter_1);
                            let state_scale_giga_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_giga_meter_10, main_access.inner_scale_giga_meter_10);
                            let state_scale_giga_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_giga_meter_100, main_access.inner_scale_giga_meter_100);
                            let state_scale_tera_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_tera_meter_1, main_access.inner_scale_tera_meter_1);
                            let state_scale_tera_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_tera_meter_10, main_access.inner_scale_tera_meter_10);
                            let state_scale_tera_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_tera_meter_100, main_access.inner_scale_tera_meter_100);
                            let state_scale_peta_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_peta_meter_1, main_access.inner_scale_peta_meter_1);
                            let state_scale_peta_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_peta_meter_10, main_access.inner_scale_peta_meter_10);
                            let state_scale_peta_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_peta_meter_100, main_access.inner_scale_peta_meter_100);
                            let state_scale_exa_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_exa_meter_1, main_access.inner_scale_exa_meter_1);
                            let state_scale_exa_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_exa_meter_10, main_access.inner_scale_exa_meter_10);
                            let state_scale_exa_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_exa_meter_100, main_access.inner_scale_exa_meter_100);
                            let state_scale_zetta_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_zetta_meter_1, main_access.inner_scale_zetta_meter_1);
                            let state_scale_zetta_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_zetta_meter_10, main_access.inner_scale_zetta_meter_10);
                            let state_scale_zetta_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_zetta_meter_100, main_access.inner_scale_zetta_meter_100);
                            let state_scale_yotta_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_yotta_meter_1, main_access.inner_scale_yotta_meter_1);
                            let state_scale_yotta_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_yotta_meter_10, main_access.inner_scale_yotta_meter_10);
                            let state_scale_yotta_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_yotta_meter_100, main_access.inner_scale_yotta_meter_100);
                            let state_scale_ronna_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_ronna_meter_1, main_access.inner_scale_ronna_meter_1);
                            let state_scale_ronna_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_ronna_meter_10, main_access.inner_scale_ronna_meter_10);
                            let state_scale_ronna_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_ronna_meter_100, main_access.inner_scale_ronna_meter_100);
                            let state_scale_quetta_meter_1 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_quetta_meter_1, main_access.inner_scale_quetta_meter_1);
                            let state_scale_quetta_meter_10 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_quetta_meter_10, main_access.inner_scale_quetta_meter_10);
                            let state_scale_quetta_meter_100 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_quetta_meter_100, main_access.inner_scale_quetta_meter_100);
                            let state_scale_quetta_meter_1000 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_quetta_meter_1000, main_access.inner_scale_quetta_meter_1000);
                            let state_scale_quetta_meter_10000 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_quetta_meter_10000, main_access.inner_scale_quetta_meter_10000);
                            let state_scale_quetta_meter_100000 = validate_and_load_and_wait_stage_setup_ecs_while(input.inner_scale_quetta_meter_100000, main_access.inner_scale_quetta_meter_100000);
                            
                            State {
                                inner_scale_quecto_meter_000001: Some(state_scale_quecto_meter_000001),
                                inner_scale_quecto_meter_00001: Some(state_scale_quecto_meter_00001),
                                inner_scale_quecto_meter_0001: Some(state_scale_quecto_meter_0001),
                                inner_scale_quecto_meter_001: Some(state_scale_quecto_meter_001),
                                inner_scale_quecto_meter_01: Some(state_scale_quecto_meter_01),
                                inner_scale_quecto_meter_1: Some(state_scale_quecto_meter_1),
                                inner_scale_quecto_meter_10: Some(state_scale_quecto_meter_10),
                                inner_scale_quecto_meter_100: Some(state_scale_quecto_meter_100),
                                inner_scale_ronto_meter_1: Some(state_scale_ronto_meter_1),
                                inner_scale_ronto_meter_10: Some(state_scale_ronto_meter_10),
                                inner_scale_ronto_meter_100: Some(state_scale_ronto_meter_100),
                                inner_scale_yocto_meter_1: Some(state_scale_yocto_meter_1),
                                inner_scale_yocto_meter_10: Some(state_scale_yocto_meter_10),
                                inner_scale_yocto_meter_100: Some(state_scale_yocto_meter_100),
                                inner_scale_zepto_meter_1: Some(state_scale_zepto_meter_1),
                                inner_scale_zepto_meter_10: Some(state_scale_zepto_meter_10),
                                inner_scale_zepto_meter_100: Some(state_scale_zepto_meter_100),
                                inner_scale_atto_meter_1: Some(state_scale_atto_meter_1),
                                inner_scale_atto_meter_10: Some(state_scale_atto_meter_10),
                                inner_scale_atto_meter_100: Some(state_scale_atto_meter_100),
                                inner_scale_femto_meter_1: Some(state_scale_femto_meter_1),
                                inner_scale_femto_meter_10: Some(state_scale_femto_meter_10),
                                inner_scale_femto_meter_100: Some(state_scale_femto_meter_100),
                                inner_scale_pico_meter_1: Some(state_scale_pico_meter_1),
                                inner_scale_pico_meter_10: Some(state_scale_pico_meter_10),
                                inner_scale_pico_meter_100: Some(state_scale_pico_meter_100),
                                inner_scale_nano_meter_1: Some(state_scale_nano_meter_1),
                                inner_scale_nano_meter_10: Some(state_scale_nano_meter_10),
                                inner_scale_nano_meter_100: Some(state_scale_nano_meter_100),
                                inner_scale_micro_meter_1: Some(state_scale_micro_meter_1),
                                inner_scale_micro_meter_10: Some(state_scale_micro_meter_10),
                                inner_scale_micro_meter_100: Some(state_scale_micro_meter_100),
                                inner_scale_milli_meter_1: Some(state_scale_milli_meter_1),
                                inner_scale_milli_meter_10: Some(state_scale_milli_meter_10),
                                inner_scale_milli_meter_100: Some(state_scale_milli_meter_100),
                                inner_scale_meter_1: Some(state_scale_meter_1),
                                inner_scale_meter_10: Some(state_scale_meter_10),
                                inner_scale_meter_100: Some(state_scale_meter_100),
                                inner_scale_kilo_meter_1: Some(state_scale_kilo_meter_1),
                                inner_scale_kilo_meter_10: Some(state_scale_kilo_meter_10),
                                inner_scale_kilo_meter_100: Some(state_scale_kilo_meter_100),
                                inner_scale_mega_meter_1: Some(state_scale_mega_meter_1),
                                inner_scale_mega_meter_10: Some(state_scale_mega_meter_10),
                                inner_scale_mega_meter_100: Some(state_scale_mega_meter_100),
                                inner_scale_giga_meter_1: Some(state_scale_giga_meter_1),
                                inner_scale_giga_meter_10: Some(state_scale_giga_meter_10),
                                inner_scale_giga_meter_100: Some(state_scale_giga_meter_100),
                                inner_scale_tera_meter_1: Some(state_scale_tera_meter_1),
                                inner_scale_tera_meter_10: Some(state_scale_tera_meter_10),
                                inner_scale_tera_meter_100: Some(state_scale_tera_meter_100),
                                inner_scale_peta_meter_1: Some(state_scale_peta_meter_1),
                                inner_scale_peta_meter_10: Some(state_scale_peta_meter_10),
                                inner_scale_peta_meter_100: Some(state_scale_peta_meter_100),
                                inner_scale_exa_meter_1: Some(state_scale_exa_meter_1),
                                inner_scale_exa_meter_10: Some(state_scale_exa_meter_10),
                                inner_scale_exa_meter_100: Some(state_scale_exa_meter_100),
                                inner_scale_zetta_meter_1: Some(state_scale_zetta_meter_1),
                                inner_scale_zetta_meter_10: Some(state_scale_zetta_meter_10),
                                inner_scale_zetta_meter_100: Some(state_scale_zetta_meter_100),
                                inner_scale_yotta_meter_1: Some(state_scale_yotta_meter_1),
                                inner_scale_yotta_meter_10: Some(state_scale_yotta_meter_10),
                                inner_scale_yotta_meter_100: Some(state_scale_yotta_meter_100),
                                inner_scale_ronna_meter_1: Some(state_scale_ronna_meter_1),
                                inner_scale_ronna_meter_10: Some(state_scale_ronna_meter_10),
                                inner_scale_ronna_meter_100: Some(state_scale_ronna_meter_100),
                                inner_scale_quetta_meter_1: Some(state_scale_quetta_meter_1),
                                inner_scale_quetta_meter_10: Some(state_scale_quetta_meter_10),
                                inner_scale_quetta_meter_100: Some(state_scale_quetta_meter_100),
                                inner_scale_quetta_meter_1000: Some(state_scale_quetta_meter_1000),
                                inner_scale_quetta_meter_10000: Some(state_scale_quetta_meter_10000),
                                inner_scale_quetta_meter_100000: Some(state_scale_quetta_meter_100000),
                            }
                        }

                        fn RunEcsWhile |state, main_access| -> Outcome<State, ()> {
                            let outcome_scale_quecto_meter_000001 = if let Some(state) = state.inner_scale_quecto_meter_000001 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quecto_meter_000001))
                            } else { None };
                            let outcome_scale_quecto_meter_00001 = if let Some(state) = state.inner_scale_quecto_meter_00001 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quecto_meter_00001))
                            } else { None };
                            let outcome_scale_quecto_meter_0001 = if let Some(state) = state.inner_scale_quecto_meter_0001 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quecto_meter_0001))
                            } else { None };
                            let outcome_scale_quecto_meter_001 = if let Some(state) = state.inner_scale_quecto_meter_001 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quecto_meter_001))
                            } else { None };
                            let outcome_scale_quecto_meter_01 = if let Some(state) = state.inner_scale_quecto_meter_01 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quecto_meter_01))
                            } else { None };
                            let outcome_scale_quecto_meter_1 = if let Some(state) = state.inner_scale_quecto_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quecto_meter_1))
                            } else { None };
                            let outcome_scale_quecto_meter_10 = if let Some(state) = state.inner_scale_quecto_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quecto_meter_10))
                            } else { None };
                            let outcome_scale_quecto_meter_100 = if let Some(state) = state.inner_scale_quecto_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quecto_meter_100))
                            } else { None };
                            let outcome_scale_ronto_meter_1 = if let Some(state) = state.inner_scale_ronto_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_ronto_meter_1))
                            } else { None };
                            let outcome_scale_ronto_meter_10 = if let Some(state) = state.inner_scale_ronto_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_ronto_meter_10))
                            } else { None };
                            let outcome_scale_ronto_meter_100 = if let Some(state) = state.inner_scale_ronto_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_ronto_meter_100))
                            } else { None };
                            let outcome_scale_yocto_meter_1 = if let Some(state) = state.inner_scale_yocto_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_yocto_meter_1))
                            } else { None };
                            let outcome_scale_yocto_meter_10 = if let Some(state) = state.inner_scale_yocto_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_yocto_meter_10))
                            } else { None };
                            let outcome_scale_yocto_meter_100 = if let Some(state) = state.inner_scale_yocto_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_yocto_meter_100))
                            } else { None };
                            let outcome_scale_zepto_meter_1 = if let Some(state) = state.inner_scale_zepto_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_zepto_meter_1))
                            } else { None };
                            let outcome_scale_zepto_meter_10 = if let Some(state) = state.inner_scale_zepto_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_zepto_meter_10))
                            } else { None };
                            let outcome_scale_zepto_meter_100 = if let Some(state) = state.inner_scale_zepto_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_zepto_meter_100))
                            } else { None };
                            let outcome_scale_atto_meter_1 = if let Some(state) = state.inner_scale_atto_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_atto_meter_1))
                            } else { None };
                            let outcome_scale_atto_meter_10 = if let Some(state) = state.inner_scale_atto_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_atto_meter_10))
                            } else { None };
                            let outcome_scale_atto_meter_100 = if let Some(state) = state.inner_scale_atto_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_atto_meter_100))
                            } else { None };
                            let outcome_scale_femto_meter_1 = if let Some(state) = state.inner_scale_femto_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_femto_meter_1))
                            } else { None };
                            let outcome_scale_femto_meter_10 = if let Some(state) = state.inner_scale_femto_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_femto_meter_10))
                            } else { None };
                            let outcome_scale_femto_meter_100 = if let Some(state) = state.inner_scale_femto_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_femto_meter_100))
                            } else { None };
                            let outcome_scale_pico_meter_1 = if let Some(state) = state.inner_scale_pico_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_pico_meter_1))
                            } else { None };
                            let outcome_scale_pico_meter_10 = if let Some(state) = state.inner_scale_pico_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_pico_meter_10))
                            } else { None };
                            let outcome_scale_pico_meter_100 = if let Some(state) = state.inner_scale_pico_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_pico_meter_100))
                            } else { None };
                            let outcome_scale_nano_meter_1 = if let Some(state) = state.inner_scale_nano_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_nano_meter_1))
                            } else { None };
                            let outcome_scale_nano_meter_10 = if let Some(state) = state.inner_scale_nano_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_nano_meter_10))
                            } else { None };
                            let outcome_scale_nano_meter_100 = if let Some(state) = state.inner_scale_nano_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_nano_meter_100))
                            } else { None };
                            let outcome_scale_micro_meter_1 = if let Some(state) = state.inner_scale_micro_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_micro_meter_1))
                            } else { None };
                            let outcome_scale_micro_meter_10 = if let Some(state) = state.inner_scale_micro_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_micro_meter_10))
                            } else { None };
                            let outcome_scale_micro_meter_100 = if let Some(state) = state.inner_scale_micro_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_micro_meter_100))
                            } else { None };
                            let outcome_scale_milli_meter_1 = if let Some(state) = state.inner_scale_milli_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_milli_meter_1))
                            } else { None };
                            let outcome_scale_milli_meter_10 = if let Some(state) = state.inner_scale_milli_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_milli_meter_10))
                            } else { None };
                            let outcome_scale_milli_meter_100 = if let Some(state) = state.inner_scale_milli_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_milli_meter_100))
                            } else { None };
                            let outcome_scale_meter_1 = if let Some(state) = state.inner_scale_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_meter_1))
                            } else { None };
                            let outcome_scale_meter_10 = if let Some(state) = state.inner_scale_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_meter_10))
                            } else { None };
                            let outcome_scale_meter_100 = if let Some(state) = state.inner_scale_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_meter_100))
                            } else { None };
                            let outcome_scale_kilo_meter_1 = if let Some(state) = state.inner_scale_kilo_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_kilo_meter_1))
                            } else { None };
                            let outcome_scale_kilo_meter_10 = if let Some(state) = state.inner_scale_kilo_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_kilo_meter_10))
                            } else { None };
                            let outcome_scale_kilo_meter_100 = if let Some(state) = state.inner_scale_kilo_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_kilo_meter_100))
                            } else { None };
                            let outcome_scale_mega_meter_1 = if let Some(state) = state.inner_scale_mega_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_mega_meter_1))
                            } else { None };
                            let outcome_scale_mega_meter_10 = if let Some(state) = state.inner_scale_mega_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_mega_meter_10))
                            } else { None };
                            let outcome_scale_mega_meter_100 = if let Some(state) = state.inner_scale_mega_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_mega_meter_100))
                            } else { None };
                            let outcome_scale_giga_meter_1 = if let Some(state) = state.inner_scale_giga_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_giga_meter_1))
                            } else { None };
                            let outcome_scale_giga_meter_10 = if let Some(state) = state.inner_scale_giga_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_giga_meter_10))
                            } else { None };
                            let outcome_scale_giga_meter_100 = if let Some(state) = state.inner_scale_giga_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_giga_meter_100))
                            } else { None };
                            let outcome_scale_tera_meter_1 = if let Some(state) = state.inner_scale_tera_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_tera_meter_1))
                            } else { None };
                            let outcome_scale_tera_meter_10 = if let Some(state) = state.inner_scale_tera_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_tera_meter_10))
                            } else { None };
                            let outcome_scale_tera_meter_100 = if let Some(state) = state.inner_scale_tera_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_tera_meter_100))
                            } else { None };
                            let outcome_scale_peta_meter_1 = if let Some(state) = state.inner_scale_peta_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_peta_meter_1))
                            } else { None };
                            let outcome_scale_peta_meter_10 = if let Some(state) = state.inner_scale_peta_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_peta_meter_10))
                            } else { None };
                            let outcome_scale_peta_meter_100 = if let Some(state) = state.inner_scale_peta_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_peta_meter_100))
                            } else { None };
                            let outcome_scale_exa_meter_1 = if let Some(state) = state.inner_scale_exa_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_exa_meter_1))
                            } else { None };
                            let outcome_scale_exa_meter_10 = if let Some(state) = state.inner_scale_exa_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_exa_meter_10))
                            } else { None };
                            let outcome_scale_exa_meter_100 = if let Some(state) = state.inner_scale_exa_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_exa_meter_100))
                            } else { None };
                            let outcome_scale_zetta_meter_1 = if let Some(state) = state.inner_scale_zetta_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_zetta_meter_1))
                            } else { None };
                            let outcome_scale_zetta_meter_10 = if let Some(state) = state.inner_scale_zetta_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_zetta_meter_10))
                            } else { None };
                            let outcome_scale_zetta_meter_100 = if let Some(state) = state.inner_scale_zetta_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_zetta_meter_100))
                            } else { None };
                            let outcome_scale_yotta_meter_1 = if let Some(state) = state.inner_scale_yotta_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_yotta_meter_1))
                            } else { None };
                            let outcome_scale_yotta_meter_10 = if let Some(state) = state.inner_scale_yotta_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_yotta_meter_10))
                            } else { None };
                            let outcome_scale_yotta_meter_100 = if let Some(state) = state.inner_scale_yotta_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_yotta_meter_100))
                            } else { None };
                            let outcome_scale_ronna_meter_1 = if let Some(state) = state.inner_scale_ronna_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_ronna_meter_1))
                            } else { None };
                            let outcome_scale_ronna_meter_10 = if let Some(state) = state.inner_scale_ronna_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_ronna_meter_10))
                            } else { None };
                            let outcome_scale_ronna_meter_100 = if let Some(state) = state.inner_scale_ronna_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_ronna_meter_100))
                            } else { None };
                            let outcome_scale_quetta_meter_1 = if let Some(state) = state.inner_scale_quetta_meter_1 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quetta_meter_1))
                            } else { None };
                            let outcome_scale_quetta_meter_10 = if let Some(state) = state.inner_scale_quetta_meter_10 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quetta_meter_10))
                            } else { None };
                            let outcome_scale_quetta_meter_100 = if let Some(state) = state.inner_scale_quetta_meter_100 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quetta_meter_100))
                            } else { None };
                            let outcome_scale_quetta_meter_1000 = if let Some(state) = state.inner_scale_quetta_meter_1000 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quetta_meter_1000))
                            } else { None };
                            let outcome_scale_quetta_meter_10000 = if let Some(state) = state.inner_scale_quetta_meter_10000 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quetta_meter_10000))
                            } else { None };
                            let outcome_scale_quetta_meter_100000 = if let Some(state) = state.inner_scale_quetta_meter_100000 {
                                Some(validate_and_load_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quetta_meter_100000))
                            } else { None };

                            if outcome_scale_quecto_meter_000001.is_none()
                                && outcome_scale_quecto_meter_00001.is_none()
                                && outcome_scale_quecto_meter_0001.is_none()
                                && outcome_scale_quecto_meter_001.is_none()
                                && outcome_scale_quecto_meter_01.is_none()
                                && outcome_scale_quecto_meter_1.is_none()
                                && outcome_scale_quecto_meter_10.is_none()
                                && outcome_scale_quecto_meter_100.is_none()
                                && outcome_scale_ronto_meter_1.is_none()
                                && outcome_scale_ronto_meter_10.is_none()
                                && outcome_scale_ronto_meter_100.is_none()
                                && outcome_scale_yocto_meter_1.is_none()
                                && outcome_scale_yocto_meter_10.is_none()
                                && outcome_scale_yocto_meter_100.is_none()
                                && outcome_scale_zepto_meter_1.is_none()
                                && outcome_scale_zepto_meter_10.is_none()
                                && outcome_scale_zepto_meter_100.is_none()
                                && outcome_scale_atto_meter_1.is_none()
                                && outcome_scale_atto_meter_10.is_none()
                                && outcome_scale_atto_meter_100.is_none()
                                && outcome_scale_femto_meter_1.is_none()
                                && outcome_scale_femto_meter_10.is_none()
                                && outcome_scale_femto_meter_100.is_none()
                                && outcome_scale_pico_meter_1.is_none()
                                && outcome_scale_pico_meter_10.is_none()
                                && outcome_scale_pico_meter_100.is_none()
                                && outcome_scale_nano_meter_1.is_none()
                                && outcome_scale_nano_meter_10.is_none()
                                && outcome_scale_nano_meter_100.is_none()
                                && outcome_scale_micro_meter_1.is_none()
                                && outcome_scale_micro_meter_10.is_none()
                                && outcome_scale_micro_meter_100.is_none()
                                && outcome_scale_milli_meter_1.is_none()
                                && outcome_scale_milli_meter_10.is_none()
                                && outcome_scale_milli_meter_100.is_none()
                                && outcome_scale_meter_1.is_none()
                                && outcome_scale_meter_10.is_none()
                                && outcome_scale_meter_100.is_none()
                                && outcome_scale_kilo_meter_1.is_none()
                                && outcome_scale_kilo_meter_10.is_none()
                                && outcome_scale_kilo_meter_100.is_none()
                                && outcome_scale_mega_meter_1.is_none()
                                && outcome_scale_mega_meter_10.is_none()
                                && outcome_scale_mega_meter_100.is_none()
                                && outcome_scale_giga_meter_1.is_none()
                                && outcome_scale_giga_meter_10.is_none()
                                && outcome_scale_giga_meter_100.is_none()
                                && outcome_scale_tera_meter_1.is_none()
                                && outcome_scale_tera_meter_10.is_none()
                                && outcome_scale_tera_meter_100.is_none()
                                && outcome_scale_peta_meter_1.is_none()
                                && outcome_scale_peta_meter_10.is_none()
                                && outcome_scale_peta_meter_100.is_none()
                                && outcome_scale_exa_meter_1.is_none()
                                && outcome_scale_exa_meter_10.is_none()
                                && outcome_scale_exa_meter_100.is_none()
                                && outcome_scale_zetta_meter_1.is_none()
                                && outcome_scale_zetta_meter_10.is_none()
                                && outcome_scale_zetta_meter_100.is_none()
                                && outcome_scale_yotta_meter_1.is_none()
                                && outcome_scale_yotta_meter_10.is_none()
                                && outcome_scale_yotta_meter_100.is_none()
                                && outcome_scale_ronna_meter_1.is_none()
                                && outcome_scale_ronna_meter_10.is_none()
                                && outcome_scale_ronna_meter_100.is_none()
                                && outcome_scale_quetta_meter_1.is_none()
                                && outcome_scale_quetta_meter_10.is_none()
                                && outcome_scale_quetta_meter_100.is_none()
                                && outcome_scale_quetta_meter_1000.is_none()
                                && outcome_scale_quetta_meter_10000.is_none()
                                && outcome_scale_quetta_meter_100000.is_none()
                            {
                                return Done(());
                            }

                            let outcome_scale_quecto_meter_000001 = match outcome_scale_quecto_meter_000001 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quecto_meter_00001 = match outcome_scale_quecto_meter_00001 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quecto_meter_0001 = match outcome_scale_quecto_meter_0001 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quecto_meter_001 = match outcome_scale_quecto_meter_001 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quecto_meter_01 = match outcome_scale_quecto_meter_01 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quecto_meter_1 = match outcome_scale_quecto_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quecto_meter_10 = match outcome_scale_quecto_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quecto_meter_100 = match outcome_scale_quecto_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_ronto_meter_1 = match outcome_scale_ronto_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_ronto_meter_10 = match outcome_scale_ronto_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_ronto_meter_100 = match outcome_scale_ronto_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_yocto_meter_1 = match outcome_scale_yocto_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_yocto_meter_10 = match outcome_scale_yocto_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_yocto_meter_100 = match outcome_scale_yocto_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_zepto_meter_1 = match outcome_scale_zepto_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_zepto_meter_10 = match outcome_scale_zepto_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_zepto_meter_100 = match outcome_scale_zepto_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_atto_meter_1 = match outcome_scale_atto_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_atto_meter_10 = match outcome_scale_atto_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_atto_meter_100 = match outcome_scale_atto_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_femto_meter_1 = match outcome_scale_femto_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_femto_meter_10 = match outcome_scale_femto_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_femto_meter_100 = match outcome_scale_femto_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_pico_meter_1 = match outcome_scale_pico_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_pico_meter_10 = match outcome_scale_pico_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_pico_meter_100 = match outcome_scale_pico_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_nano_meter_1 = match outcome_scale_nano_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_nano_meter_10 = match outcome_scale_nano_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_nano_meter_100 = match outcome_scale_nano_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_micro_meter_1 = match outcome_scale_micro_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_micro_meter_10 = match outcome_scale_micro_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_micro_meter_100 = match outcome_scale_micro_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_milli_meter_1 = match outcome_scale_milli_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_milli_meter_10 = match outcome_scale_milli_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_milli_meter_100 = match outcome_scale_milli_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_meter_1 = match outcome_scale_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_meter_10 = match outcome_scale_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_meter_100 = match outcome_scale_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_kilo_meter_1 = match outcome_scale_kilo_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_kilo_meter_10 = match outcome_scale_kilo_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_kilo_meter_100 = match outcome_scale_kilo_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_mega_meter_1 = match outcome_scale_mega_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_mega_meter_10 = match outcome_scale_mega_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_mega_meter_100 = match outcome_scale_mega_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_giga_meter_1 = match outcome_scale_giga_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_giga_meter_10 = match outcome_scale_giga_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_giga_meter_100 = match outcome_scale_giga_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_tera_meter_1 = match outcome_scale_tera_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_tera_meter_10 = match outcome_scale_tera_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_tera_meter_100 = match outcome_scale_tera_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_peta_meter_1 = match outcome_scale_peta_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_peta_meter_10 = match outcome_scale_peta_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_peta_meter_100 = match outcome_scale_peta_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_exa_meter_1 = match outcome_scale_exa_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_exa_meter_10 = match outcome_scale_exa_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_exa_meter_100 = match outcome_scale_exa_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_zetta_meter_1 = match outcome_scale_zetta_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_zetta_meter_10 = match outcome_scale_zetta_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_zetta_meter_100 = match outcome_scale_zetta_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_yotta_meter_1 = match outcome_scale_yotta_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_yotta_meter_10 = match outcome_scale_yotta_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_yotta_meter_100 = match outcome_scale_yotta_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_ronna_meter_1 = match outcome_scale_ronna_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_ronna_meter_10 = match outcome_scale_ronna_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_ronna_meter_100 = match outcome_scale_ronna_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quetta_meter_1 = match outcome_scale_quetta_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quetta_meter_10 = match outcome_scale_quetta_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quetta_meter_100 = match outcome_scale_quetta_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quetta_meter_1000 = match outcome_scale_quetta_meter_1000 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quetta_meter_10000 = match outcome_scale_quetta_meter_10000 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quetta_meter_100000 = match outcome_scale_quetta_meter_100000 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };

                            Wait(State {
                                inner_scale_quecto_meter_000001: outcome_scale_quecto_meter_000001,
                                inner_scale_quecto_meter_00001: outcome_scale_quecto_meter_00001,
                                inner_scale_quecto_meter_0001: outcome_scale_quecto_meter_0001,
                                inner_scale_quecto_meter_001: outcome_scale_quecto_meter_001,
                                inner_scale_quecto_meter_01: outcome_scale_quecto_meter_01,
                                inner_scale_quecto_meter_1: outcome_scale_quecto_meter_1,
                                inner_scale_quecto_meter_10: outcome_scale_quecto_meter_10,
                                inner_scale_quecto_meter_100: outcome_scale_quecto_meter_100,
                                inner_scale_ronto_meter_1: outcome_scale_ronto_meter_1,
                                inner_scale_ronto_meter_10: outcome_scale_ronto_meter_10,
                                inner_scale_ronto_meter_100: outcome_scale_ronto_meter_100,
                                inner_scale_yocto_meter_1: outcome_scale_yocto_meter_1,
                                inner_scale_yocto_meter_10: outcome_scale_yocto_meter_10,
                                inner_scale_yocto_meter_100: outcome_scale_yocto_meter_100,
                                inner_scale_zepto_meter_1: outcome_scale_zepto_meter_1,
                                inner_scale_zepto_meter_10: outcome_scale_zepto_meter_10,
                                inner_scale_zepto_meter_100: outcome_scale_zepto_meter_100,
                                inner_scale_atto_meter_1: outcome_scale_atto_meter_1,
                                inner_scale_atto_meter_10: outcome_scale_atto_meter_10,
                                inner_scale_atto_meter_100: outcome_scale_atto_meter_100,
                                inner_scale_femto_meter_1: outcome_scale_femto_meter_1,
                                inner_scale_femto_meter_10: outcome_scale_femto_meter_10,
                                inner_scale_femto_meter_100: outcome_scale_femto_meter_100,
                                inner_scale_pico_meter_1: outcome_scale_pico_meter_1,
                                inner_scale_pico_meter_10: outcome_scale_pico_meter_10,
                                inner_scale_pico_meter_100: outcome_scale_pico_meter_100,
                                inner_scale_nano_meter_1: outcome_scale_nano_meter_1,
                                inner_scale_nano_meter_10: outcome_scale_nano_meter_10,
                                inner_scale_nano_meter_100: outcome_scale_nano_meter_100,
                                inner_scale_micro_meter_1: outcome_scale_micro_meter_1,
                                inner_scale_micro_meter_10: outcome_scale_micro_meter_10,
                                inner_scale_micro_meter_100: outcome_scale_micro_meter_100,
                                inner_scale_milli_meter_1: outcome_scale_milli_meter_1,
                                inner_scale_milli_meter_10: outcome_scale_milli_meter_10,
                                inner_scale_milli_meter_100: outcome_scale_milli_meter_100,
                                inner_scale_meter_1: outcome_scale_meter_1,
                                inner_scale_meter_10: outcome_scale_meter_10,
                                inner_scale_meter_100: outcome_scale_meter_100,
                                inner_scale_kilo_meter_1: outcome_scale_kilo_meter_1,
                                inner_scale_kilo_meter_10: outcome_scale_kilo_meter_10,
                                inner_scale_kilo_meter_100: outcome_scale_kilo_meter_100,
                                inner_scale_mega_meter_1: outcome_scale_mega_meter_1,
                                inner_scale_mega_meter_10: outcome_scale_mega_meter_10,
                                inner_scale_mega_meter_100: outcome_scale_mega_meter_100,
                                inner_scale_giga_meter_1: outcome_scale_giga_meter_1,
                                inner_scale_giga_meter_10: outcome_scale_giga_meter_10,
                                inner_scale_giga_meter_100: outcome_scale_giga_meter_100,
                                inner_scale_tera_meter_1: outcome_scale_tera_meter_1,
                                inner_scale_tera_meter_10: outcome_scale_tera_meter_10,
                                inner_scale_tera_meter_100: outcome_scale_tera_meter_100,
                                inner_scale_peta_meter_1: outcome_scale_peta_meter_1,
                                inner_scale_peta_meter_10: outcome_scale_peta_meter_10,
                                inner_scale_peta_meter_100: outcome_scale_peta_meter_100,
                                inner_scale_exa_meter_1: outcome_scale_exa_meter_1,
                                inner_scale_exa_meter_10: outcome_scale_exa_meter_10,
                                inner_scale_exa_meter_100: outcome_scale_exa_meter_100,
                                inner_scale_zetta_meter_1: outcome_scale_zetta_meter_1,
                                inner_scale_zetta_meter_10: outcome_scale_zetta_meter_10,
                                inner_scale_zetta_meter_100: outcome_scale_zetta_meter_100,
                                inner_scale_yotta_meter_1: outcome_scale_yotta_meter_1,
                                inner_scale_yotta_meter_10: outcome_scale_yotta_meter_10,
                                inner_scale_yotta_meter_100: outcome_scale_yotta_meter_100,
                                inner_scale_ronna_meter_1: outcome_scale_ronna_meter_1,
                                inner_scale_ronna_meter_10: outcome_scale_ronna_meter_10,
                                inner_scale_ronna_meter_100: outcome_scale_ronna_meter_100,
                                inner_scale_quetta_meter_1: outcome_scale_quetta_meter_1,
                                inner_scale_quetta_meter_10: outcome_scale_quetta_meter_10,
                                inner_scale_quetta_meter_100: outcome_scale_quetta_meter_100,
                                inner_scale_quetta_meter_1000: outcome_scale_quetta_meter_1000,
                                inner_scale_quetta_meter_10000: outcome_scale_quetta_meter_10000,
                                inner_scale_quetta_meter_100000: outcome_scale_quetta_meter_100000,
                            })
                        }
                    ]
                }
            ]
        }

        UnloadChunks, timeout_secs: 1.0, timeout_mode: VirtualTime {
            user_imports: {
                use bevy::prelude::ResMut;
                
                use crate::chunk_loader::workflows::external::unload_chunks::{
                    MainAccess as UnloadAndWaitStageMainAccess,
                    Input as UnloadAndWaitStageInput,
                    State as UnloadAndWaitStageState,
                    setup_ecs_while as unload_and_wait_stage_setup_ecs_while,
                    run_ecs_while as unload_and_wait_stage_run_ecs_while,
                };
                use crate::usf::scale::*;
            },
            user_items: {
            },
            stages: [
                UnloadAndWait: EcsWhile, run_if_paused: false, run_after_startup_finished: true {
                    core_types: [
                        struct MainAccess<'w, 's> {
                            inner_scale_quecto_meter_000001: UnloadAndWaitStageMainAccess<'w, 's, ScaleQuectoMeter000001>,
                            inner_scale_quecto_meter_00001: UnloadAndWaitStageMainAccess<'w, 's, ScaleQuectoMeter00001>,
                            inner_scale_quecto_meter_0001: UnloadAndWaitStageMainAccess<'w, 's, ScaleQuectoMeter0001>,
                            inner_scale_quecto_meter_001: UnloadAndWaitStageMainAccess<'w, 's, ScaleQuectoMeter001>,
                            inner_scale_quecto_meter_01: UnloadAndWaitStageMainAccess<'w, 's, ScaleQuectoMeter01>,
                            inner_scale_quecto_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScaleQuectoMeter1>,
                            inner_scale_quecto_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScaleQuectoMeter10>,
                            inner_scale_quecto_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScaleQuectoMeter100>,
                            inner_scale_ronto_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScaleRontoMeter1>,
                            inner_scale_ronto_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScaleRontoMeter10>,
                            inner_scale_ronto_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScaleRontoMeter100>,
                            inner_scale_yocto_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScaleYoctoMeter1>,
                            inner_scale_yocto_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScaleYoctoMeter10>,
                            inner_scale_yocto_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScaleYoctoMeter100>,
                            inner_scale_zepto_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScaleZeptoMeter1>,
                            inner_scale_zepto_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScaleZeptoMeter10>,
                            inner_scale_zepto_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScaleZeptoMeter100>,
                            inner_scale_atto_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScaleAttoMeter1>,
                            inner_scale_atto_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScaleAttoMeter10>,
                            inner_scale_atto_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScaleAttoMeter100>,
                            inner_scale_femto_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScaleFemtoMeter1>,
                            inner_scale_femto_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScaleFemtoMeter10>,
                            inner_scale_femto_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScaleFemtoMeter100>,
                            inner_scale_pico_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScalePicoMeter1>,
                            inner_scale_pico_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScalePicoMeter10>,
                            inner_scale_pico_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScalePicoMeter100>,
                            inner_scale_nano_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScaleNanoMeter1>,
                            inner_scale_nano_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScaleNanoMeter10>,
                            inner_scale_nano_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScaleNanoMeter100>,
                            inner_scale_micro_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScaleMicroMeter1>,
                            inner_scale_micro_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScaleMicroMeter10>,
                            inner_scale_micro_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScaleMicroMeter100>,
                            inner_scale_milli_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScaleMilliMeter1>,
                            inner_scale_milli_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScaleMilliMeter10>,
                            inner_scale_milli_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScaleMilliMeter100>,
                            inner_scale_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScaleMeter1>,
                            inner_scale_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScaleMeter10>,
                            inner_scale_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScaleMeter100>,
                            inner_scale_kilo_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScaleKiloMeter1>,
                            inner_scale_kilo_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScaleKiloMeter10>,
                            inner_scale_kilo_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScaleKiloMeter100>,
                            inner_scale_mega_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScaleMegaMeter1>,
                            inner_scale_mega_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScaleMegaMeter10>,
                            inner_scale_mega_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScaleMegaMeter100>,
                            inner_scale_giga_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScaleGigaMeter1>,
                            inner_scale_giga_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScaleGigaMeter10>,
                            inner_scale_giga_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScaleGigaMeter100>,
                            inner_scale_tera_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScaleTeraMeter1>,
                            inner_scale_tera_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScaleTeraMeter10>,
                            inner_scale_tera_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScaleTeraMeter100>,
                            inner_scale_peta_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScalePetaMeter1>,
                            inner_scale_peta_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScalePetaMeter10>,
                            inner_scale_peta_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScalePetaMeter100>,
                            inner_scale_exa_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScaleExaMeter1>,
                            inner_scale_exa_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScaleExaMeter10>,
                            inner_scale_exa_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScaleExaMeter100>,
                            inner_scale_zetta_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScaleZettaMeter1>,
                            inner_scale_zetta_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScaleZettaMeter10>,
                            inner_scale_zetta_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScaleZettaMeter100>,
                            inner_scale_yotta_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScaleYottaMeter1>,
                            inner_scale_yotta_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScaleYottaMeter10>,
                            inner_scale_yotta_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScaleYottaMeter100>,
                            inner_scale_ronna_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScaleRonnaMeter1>,
                            inner_scale_ronna_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScaleRonnaMeter10>,
                            inner_scale_ronna_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScaleRonnaMeter100>,
                            inner_scale_quetta_meter_1: UnloadAndWaitStageMainAccess<'w, 's, ScaleQuettaMeter1>,
                            inner_scale_quetta_meter_10: UnloadAndWaitStageMainAccess<'w, 's, ScaleQuettaMeter10>,
                            inner_scale_quetta_meter_100: UnloadAndWaitStageMainAccess<'w, 's, ScaleQuettaMeter100>,
                            inner_scale_quetta_meter_1000: UnloadAndWaitStageMainAccess<'w, 's, ScaleQuettaMeter1000>,
                            inner_scale_quetta_meter_10000: UnloadAndWaitStageMainAccess<'w, 's, ScaleQuettaMeter10000>,
                            inner_scale_quetta_meter_100000: UnloadAndWaitStageMainAccess<'w, 's, ScaleQuettaMeter100000>,
                        }
                        struct Input {
                            inner_scale_quecto_meter_000001: UnloadAndWaitStageInput<ScaleQuectoMeter000001>,
                            inner_scale_quecto_meter_00001: UnloadAndWaitStageInput<ScaleQuectoMeter00001>,
                            inner_scale_quecto_meter_0001: UnloadAndWaitStageInput<ScaleQuectoMeter0001>,
                            inner_scale_quecto_meter_001: UnloadAndWaitStageInput<ScaleQuectoMeter001>,
                            inner_scale_quecto_meter_01: UnloadAndWaitStageInput<ScaleQuectoMeter01>,
                            inner_scale_quecto_meter_1: UnloadAndWaitStageInput<ScaleQuectoMeter1>,
                            inner_scale_quecto_meter_10: UnloadAndWaitStageInput<ScaleQuectoMeter10>,
                            inner_scale_quecto_meter_100: UnloadAndWaitStageInput<ScaleQuectoMeter100>,
                            inner_scale_ronto_meter_1: UnloadAndWaitStageInput<ScaleRontoMeter1>,
                            inner_scale_ronto_meter_10: UnloadAndWaitStageInput<ScaleRontoMeter10>,
                            inner_scale_ronto_meter_100: UnloadAndWaitStageInput<ScaleRontoMeter100>,
                            inner_scale_yocto_meter_1: UnloadAndWaitStageInput<ScaleYoctoMeter1>,
                            inner_scale_yocto_meter_10: UnloadAndWaitStageInput<ScaleYoctoMeter10>,
                            inner_scale_yocto_meter_100: UnloadAndWaitStageInput<ScaleYoctoMeter100>,
                            inner_scale_zepto_meter_1: UnloadAndWaitStageInput<ScaleZeptoMeter1>,
                            inner_scale_zepto_meter_10: UnloadAndWaitStageInput<ScaleZeptoMeter10>,
                            inner_scale_zepto_meter_100: UnloadAndWaitStageInput<ScaleZeptoMeter100>,
                            inner_scale_atto_meter_1: UnloadAndWaitStageInput<ScaleAttoMeter1>,
                            inner_scale_atto_meter_10: UnloadAndWaitStageInput<ScaleAttoMeter10>,
                            inner_scale_atto_meter_100: UnloadAndWaitStageInput<ScaleAttoMeter100>,
                            inner_scale_femto_meter_1: UnloadAndWaitStageInput<ScaleFemtoMeter1>,
                            inner_scale_femto_meter_10: UnloadAndWaitStageInput<ScaleFemtoMeter10>,
                            inner_scale_femto_meter_100: UnloadAndWaitStageInput<ScaleFemtoMeter100>,
                            inner_scale_pico_meter_1: UnloadAndWaitStageInput<ScalePicoMeter1>,
                            inner_scale_pico_meter_10: UnloadAndWaitStageInput<ScalePicoMeter10>,
                            inner_scale_pico_meter_100: UnloadAndWaitStageInput<ScalePicoMeter100>,
                            inner_scale_nano_meter_1: UnloadAndWaitStageInput<ScaleNanoMeter1>,
                            inner_scale_nano_meter_10: UnloadAndWaitStageInput<ScaleNanoMeter10>,
                            inner_scale_nano_meter_100: UnloadAndWaitStageInput<ScaleNanoMeter100>,
                            inner_scale_micro_meter_1: UnloadAndWaitStageInput<ScaleMicroMeter1>,
                            inner_scale_micro_meter_10: UnloadAndWaitStageInput<ScaleMicroMeter10>,
                            inner_scale_micro_meter_100: UnloadAndWaitStageInput<ScaleMicroMeter100>,
                            inner_scale_milli_meter_1: UnloadAndWaitStageInput<ScaleMilliMeter1>,
                            inner_scale_milli_meter_10: UnloadAndWaitStageInput<ScaleMilliMeter10>,
                            inner_scale_milli_meter_100: UnloadAndWaitStageInput<ScaleMilliMeter100>,
                            inner_scale_meter_1: UnloadAndWaitStageInput<ScaleMeter1>,
                            inner_scale_meter_10: UnloadAndWaitStageInput<ScaleMeter10>,
                            inner_scale_meter_100: UnloadAndWaitStageInput<ScaleMeter100>,
                            inner_scale_kilo_meter_1: UnloadAndWaitStageInput<ScaleKiloMeter1>,
                            inner_scale_kilo_meter_10: UnloadAndWaitStageInput<ScaleKiloMeter10>,
                            inner_scale_kilo_meter_100: UnloadAndWaitStageInput<ScaleKiloMeter100>,
                            inner_scale_mega_meter_1: UnloadAndWaitStageInput<ScaleMegaMeter1>,
                            inner_scale_mega_meter_10: UnloadAndWaitStageInput<ScaleMegaMeter10>,
                            inner_scale_mega_meter_100: UnloadAndWaitStageInput<ScaleMegaMeter100>,
                            inner_scale_giga_meter_1: UnloadAndWaitStageInput<ScaleGigaMeter1>,
                            inner_scale_giga_meter_10: UnloadAndWaitStageInput<ScaleGigaMeter10>,
                            inner_scale_giga_meter_100: UnloadAndWaitStageInput<ScaleGigaMeter100>,
                            inner_scale_tera_meter_1: UnloadAndWaitStageInput<ScaleTeraMeter1>,
                            inner_scale_tera_meter_10: UnloadAndWaitStageInput<ScaleTeraMeter10>,
                            inner_scale_tera_meter_100: UnloadAndWaitStageInput<ScaleTeraMeter100>,
                            inner_scale_peta_meter_1: UnloadAndWaitStageInput<ScalePetaMeter1>,
                            inner_scale_peta_meter_10: UnloadAndWaitStageInput<ScalePetaMeter10>,
                            inner_scale_peta_meter_100: UnloadAndWaitStageInput<ScalePetaMeter100>,
                            inner_scale_exa_meter_1: UnloadAndWaitStageInput<ScaleExaMeter1>,
                            inner_scale_exa_meter_10: UnloadAndWaitStageInput<ScaleExaMeter10>,
                            inner_scale_exa_meter_100: UnloadAndWaitStageInput<ScaleExaMeter100>,
                            inner_scale_zetta_meter_1: UnloadAndWaitStageInput<ScaleZettaMeter1>,
                            inner_scale_zetta_meter_10: UnloadAndWaitStageInput<ScaleZettaMeter10>,
                            inner_scale_zetta_meter_100: UnloadAndWaitStageInput<ScaleZettaMeter100>,
                            inner_scale_yotta_meter_1: UnloadAndWaitStageInput<ScaleYottaMeter1>,
                            inner_scale_yotta_meter_10: UnloadAndWaitStageInput<ScaleYottaMeter10>,
                            inner_scale_yotta_meter_100: UnloadAndWaitStageInput<ScaleYottaMeter100>,
                            inner_scale_ronna_meter_1: UnloadAndWaitStageInput<ScaleRonnaMeter1>,
                            inner_scale_ronna_meter_10: UnloadAndWaitStageInput<ScaleRonnaMeter10>,
                            inner_scale_ronna_meter_100: UnloadAndWaitStageInput<ScaleRonnaMeter100>,
                            inner_scale_quetta_meter_1: UnloadAndWaitStageInput<ScaleQuettaMeter1>,
                            inner_scale_quetta_meter_10: UnloadAndWaitStageInput<ScaleQuettaMeter10>,
                            inner_scale_quetta_meter_100: UnloadAndWaitStageInput<ScaleQuettaMeter100>,
                            inner_scale_quetta_meter_1000: UnloadAndWaitStageInput<ScaleQuettaMeter1000>,
                            inner_scale_quetta_meter_10000: UnloadAndWaitStageInput<ScaleQuettaMeter10000>,
                            inner_scale_quetta_meter_100000: UnloadAndWaitStageInput<ScaleQuettaMeter100000>,
                        }
                        struct State {
                            inner_scale_quecto_meter_000001: Option<UnloadAndWaitStageState<ScaleQuectoMeter000001>>,
                            inner_scale_quecto_meter_00001: Option<UnloadAndWaitStageState<ScaleQuectoMeter00001>>,
                            inner_scale_quecto_meter_0001: Option<UnloadAndWaitStageState<ScaleQuectoMeter0001>>,
                            inner_scale_quecto_meter_001: Option<UnloadAndWaitStageState<ScaleQuectoMeter001>>,
                            inner_scale_quecto_meter_01: Option<UnloadAndWaitStageState<ScaleQuectoMeter01>>,
                            inner_scale_quecto_meter_1: Option<UnloadAndWaitStageState<ScaleQuectoMeter1>>,
                            inner_scale_quecto_meter_10: Option<UnloadAndWaitStageState<ScaleQuectoMeter10>>,
                            inner_scale_quecto_meter_100: Option<UnloadAndWaitStageState<ScaleQuectoMeter100>>,
                            inner_scale_ronto_meter_1: Option<UnloadAndWaitStageState<ScaleRontoMeter1>>,
                            inner_scale_ronto_meter_10: Option<UnloadAndWaitStageState<ScaleRontoMeter10>>,
                            inner_scale_ronto_meter_100: Option<UnloadAndWaitStageState<ScaleRontoMeter100>>,
                            inner_scale_yocto_meter_1: Option<UnloadAndWaitStageState<ScaleYoctoMeter1>>,
                            inner_scale_yocto_meter_10: Option<UnloadAndWaitStageState<ScaleYoctoMeter10>>,
                            inner_scale_yocto_meter_100: Option<UnloadAndWaitStageState<ScaleYoctoMeter100>>,
                            inner_scale_zepto_meter_1: Option<UnloadAndWaitStageState<ScaleZeptoMeter1>>,
                            inner_scale_zepto_meter_10: Option<UnloadAndWaitStageState<ScaleZeptoMeter10>>,
                            inner_scale_zepto_meter_100: Option<UnloadAndWaitStageState<ScaleZeptoMeter100>>,
                            inner_scale_atto_meter_1: Option<UnloadAndWaitStageState<ScaleAttoMeter1>>,
                            inner_scale_atto_meter_10: Option<UnloadAndWaitStageState<ScaleAttoMeter10>>,
                            inner_scale_atto_meter_100: Option<UnloadAndWaitStageState<ScaleAttoMeter100>>,
                            inner_scale_femto_meter_1: Option<UnloadAndWaitStageState<ScaleFemtoMeter1>>,
                            inner_scale_femto_meter_10: Option<UnloadAndWaitStageState<ScaleFemtoMeter10>>,
                            inner_scale_femto_meter_100: Option<UnloadAndWaitStageState<ScaleFemtoMeter100>>,
                            inner_scale_pico_meter_1: Option<UnloadAndWaitStageState<ScalePicoMeter1>>,
                            inner_scale_pico_meter_10: Option<UnloadAndWaitStageState<ScalePicoMeter10>>,
                            inner_scale_pico_meter_100: Option<UnloadAndWaitStageState<ScalePicoMeter100>>,
                            inner_scale_nano_meter_1: Option<UnloadAndWaitStageState<ScaleNanoMeter1>>,
                            inner_scale_nano_meter_10: Option<UnloadAndWaitStageState<ScaleNanoMeter10>>,
                            inner_scale_nano_meter_100: Option<UnloadAndWaitStageState<ScaleNanoMeter100>>,
                            inner_scale_micro_meter_1: Option<UnloadAndWaitStageState<ScaleMicroMeter1>>,
                            inner_scale_micro_meter_10: Option<UnloadAndWaitStageState<ScaleMicroMeter10>>,
                            inner_scale_micro_meter_100: Option<UnloadAndWaitStageState<ScaleMicroMeter100>>,
                            inner_scale_milli_meter_1: Option<UnloadAndWaitStageState<ScaleMilliMeter1>>,
                            inner_scale_milli_meter_10: Option<UnloadAndWaitStageState<ScaleMilliMeter10>>,
                            inner_scale_milli_meter_100: Option<UnloadAndWaitStageState<ScaleMilliMeter100>>,
                            inner_scale_meter_1: Option<UnloadAndWaitStageState<ScaleMeter1>>,
                            inner_scale_meter_10: Option<UnloadAndWaitStageState<ScaleMeter10>>,
                            inner_scale_meter_100: Option<UnloadAndWaitStageState<ScaleMeter100>>,
                            inner_scale_kilo_meter_1: Option<UnloadAndWaitStageState<ScaleKiloMeter1>>,
                            inner_scale_kilo_meter_10: Option<UnloadAndWaitStageState<ScaleKiloMeter10>>,
                            inner_scale_kilo_meter_100: Option<UnloadAndWaitStageState<ScaleKiloMeter100>>,
                            inner_scale_mega_meter_1: Option<UnloadAndWaitStageState<ScaleMegaMeter1>>,
                            inner_scale_mega_meter_10: Option<UnloadAndWaitStageState<ScaleMegaMeter10>>,
                            inner_scale_mega_meter_100: Option<UnloadAndWaitStageState<ScaleMegaMeter100>>,
                            inner_scale_giga_meter_1: Option<UnloadAndWaitStageState<ScaleGigaMeter1>>,
                            inner_scale_giga_meter_10: Option<UnloadAndWaitStageState<ScaleGigaMeter10>>,
                            inner_scale_giga_meter_100: Option<UnloadAndWaitStageState<ScaleGigaMeter100>>,
                            inner_scale_tera_meter_1: Option<UnloadAndWaitStageState<ScaleTeraMeter1>>,
                            inner_scale_tera_meter_10: Option<UnloadAndWaitStageState<ScaleTeraMeter10>>,
                            inner_scale_tera_meter_100: Option<UnloadAndWaitStageState<ScaleTeraMeter100>>,
                            inner_scale_peta_meter_1: Option<UnloadAndWaitStageState<ScalePetaMeter1>>,
                            inner_scale_peta_meter_10: Option<UnloadAndWaitStageState<ScalePetaMeter10>>,
                            inner_scale_peta_meter_100: Option<UnloadAndWaitStageState<ScalePetaMeter100>>,
                            inner_scale_exa_meter_1: Option<UnloadAndWaitStageState<ScaleExaMeter1>>,
                            inner_scale_exa_meter_10: Option<UnloadAndWaitStageState<ScaleExaMeter10>>,
                            inner_scale_exa_meter_100: Option<UnloadAndWaitStageState<ScaleExaMeter100>>,
                            inner_scale_zetta_meter_1: Option<UnloadAndWaitStageState<ScaleZettaMeter1>>,
                            inner_scale_zetta_meter_10: Option<UnloadAndWaitStageState<ScaleZettaMeter10>>,
                            inner_scale_zetta_meter_100: Option<UnloadAndWaitStageState<ScaleZettaMeter100>>,
                            inner_scale_yotta_meter_1: Option<UnloadAndWaitStageState<ScaleYottaMeter1>>,
                            inner_scale_yotta_meter_10: Option<UnloadAndWaitStageState<ScaleYottaMeter10>>,
                            inner_scale_yotta_meter_100: Option<UnloadAndWaitStageState<ScaleYottaMeter100>>,
                            inner_scale_ronna_meter_1: Option<UnloadAndWaitStageState<ScaleRonnaMeter1>>,
                            inner_scale_ronna_meter_10: Option<UnloadAndWaitStageState<ScaleRonnaMeter10>>,
                            inner_scale_ronna_meter_100: Option<UnloadAndWaitStageState<ScaleRonnaMeter100>>,
                            inner_scale_quetta_meter_1: Option<UnloadAndWaitStageState<ScaleQuettaMeter1>>,
                            inner_scale_quetta_meter_10: Option<UnloadAndWaitStageState<ScaleQuettaMeter10>>,
                            inner_scale_quetta_meter_100: Option<UnloadAndWaitStageState<ScaleQuettaMeter100>>,
                            inner_scale_quetta_meter_1000: Option<UnloadAndWaitStageState<ScaleQuettaMeter1000>>,
                            inner_scale_quetta_meter_10000: Option<UnloadAndWaitStageState<ScaleQuettaMeter10000>>,
                            inner_scale_quetta_meter_100000: Option<UnloadAndWaitStageState<ScaleQuettaMeter100000>>,
                        }
                    ],
                    core_functions: [
                        fn SetupEcsWhile |input, main_access| -> State {
                            let state_scale_quecto_meter_000001 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_quecto_meter_000001, main_access.inner_scale_quecto_meter_000001);
                            let state_scale_quecto_meter_00001 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_quecto_meter_00001, main_access.inner_scale_quecto_meter_00001);
                            let state_scale_quecto_meter_0001 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_quecto_meter_0001, main_access.inner_scale_quecto_meter_0001);
                            let state_scale_quecto_meter_001 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_quecto_meter_001, main_access.inner_scale_quecto_meter_001);
                            let state_scale_quecto_meter_01 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_quecto_meter_01, main_access.inner_scale_quecto_meter_01);
                            let state_scale_quecto_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_quecto_meter_1, main_access.inner_scale_quecto_meter_1);
                            let state_scale_quecto_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_quecto_meter_10, main_access.inner_scale_quecto_meter_10);
                            let state_scale_quecto_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_quecto_meter_100, main_access.inner_scale_quecto_meter_100);
                            let state_scale_ronto_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_ronto_meter_1, main_access.inner_scale_ronto_meter_1);
                            let state_scale_ronto_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_ronto_meter_10, main_access.inner_scale_ronto_meter_10);
                            let state_scale_ronto_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_ronto_meter_100, main_access.inner_scale_ronto_meter_100);
                            let state_scale_yocto_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_yocto_meter_1, main_access.inner_scale_yocto_meter_1);
                            let state_scale_yocto_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_yocto_meter_10, main_access.inner_scale_yocto_meter_10);
                            let state_scale_yocto_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_yocto_meter_100, main_access.inner_scale_yocto_meter_100);
                            let state_scale_zepto_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_zepto_meter_1, main_access.inner_scale_zepto_meter_1);
                            let state_scale_zepto_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_zepto_meter_10, main_access.inner_scale_zepto_meter_10);
                            let state_scale_zepto_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_zepto_meter_100, main_access.inner_scale_zepto_meter_100);
                            let state_scale_atto_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_atto_meter_1, main_access.inner_scale_atto_meter_1);
                            let state_scale_atto_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_atto_meter_10, main_access.inner_scale_atto_meter_10);
                            let state_scale_atto_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_atto_meter_100, main_access.inner_scale_atto_meter_100);
                            let state_scale_femto_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_femto_meter_1, main_access.inner_scale_femto_meter_1);
                            let state_scale_femto_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_femto_meter_10, main_access.inner_scale_femto_meter_10);
                            let state_scale_femto_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_femto_meter_100, main_access.inner_scale_femto_meter_100);
                            let state_scale_pico_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_pico_meter_1, main_access.inner_scale_pico_meter_1);
                            let state_scale_pico_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_pico_meter_10, main_access.inner_scale_pico_meter_10);
                            let state_scale_pico_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_pico_meter_100, main_access.inner_scale_pico_meter_100);
                            let state_scale_nano_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_nano_meter_1, main_access.inner_scale_nano_meter_1);
                            let state_scale_nano_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_nano_meter_10, main_access.inner_scale_nano_meter_10);
                            let state_scale_nano_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_nano_meter_100, main_access.inner_scale_nano_meter_100);
                            let state_scale_micro_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_micro_meter_1, main_access.inner_scale_micro_meter_1);
                            let state_scale_micro_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_micro_meter_10, main_access.inner_scale_micro_meter_10);
                            let state_scale_micro_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_micro_meter_100, main_access.inner_scale_micro_meter_100);
                            let state_scale_milli_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_milli_meter_1, main_access.inner_scale_milli_meter_1);
                            let state_scale_milli_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_milli_meter_10, main_access.inner_scale_milli_meter_10);
                            let state_scale_milli_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_milli_meter_100, main_access.inner_scale_milli_meter_100);
                            let state_scale_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_meter_1, main_access.inner_scale_meter_1);
                            let state_scale_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_meter_10, main_access.inner_scale_meter_10);
                            let state_scale_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_meter_100, main_access.inner_scale_meter_100);
                            let state_scale_kilo_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_kilo_meter_1, main_access.inner_scale_kilo_meter_1);
                            let state_scale_kilo_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_kilo_meter_10, main_access.inner_scale_kilo_meter_10);
                            let state_scale_kilo_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_kilo_meter_100, main_access.inner_scale_kilo_meter_100);
                            let state_scale_mega_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_mega_meter_1, main_access.inner_scale_mega_meter_1);
                            let state_scale_mega_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_mega_meter_10, main_access.inner_scale_mega_meter_10);
                            let state_scale_mega_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_mega_meter_100, main_access.inner_scale_mega_meter_100);
                            let state_scale_giga_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_giga_meter_1, main_access.inner_scale_giga_meter_1);
                            let state_scale_giga_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_giga_meter_10, main_access.inner_scale_giga_meter_10);
                            let state_scale_giga_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_giga_meter_100, main_access.inner_scale_giga_meter_100);
                            let state_scale_tera_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_tera_meter_1, main_access.inner_scale_tera_meter_1);
                            let state_scale_tera_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_tera_meter_10, main_access.inner_scale_tera_meter_10);
                            let state_scale_tera_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_tera_meter_100, main_access.inner_scale_tera_meter_100);
                            let state_scale_peta_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_peta_meter_1, main_access.inner_scale_peta_meter_1);
                            let state_scale_peta_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_peta_meter_10, main_access.inner_scale_peta_meter_10);
                            let state_scale_peta_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_peta_meter_100, main_access.inner_scale_peta_meter_100);
                            let state_scale_exa_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_exa_meter_1, main_access.inner_scale_exa_meter_1);
                            let state_scale_exa_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_exa_meter_10, main_access.inner_scale_exa_meter_10);
                            let state_scale_exa_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_exa_meter_100, main_access.inner_scale_exa_meter_100);
                            let state_scale_zetta_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_zetta_meter_1, main_access.inner_scale_zetta_meter_1);
                            let state_scale_zetta_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_zetta_meter_10, main_access.inner_scale_zetta_meter_10);
                            let state_scale_zetta_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_zetta_meter_100, main_access.inner_scale_zetta_meter_100);
                            let state_scale_yotta_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_yotta_meter_1, main_access.inner_scale_yotta_meter_1);
                            let state_scale_yotta_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_yotta_meter_10, main_access.inner_scale_yotta_meter_10);
                            let state_scale_yotta_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_yotta_meter_100, main_access.inner_scale_yotta_meter_100);
                            let state_scale_ronna_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_ronna_meter_1, main_access.inner_scale_ronna_meter_1);
                            let state_scale_ronna_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_ronna_meter_10, main_access.inner_scale_ronna_meter_10);
                            let state_scale_ronna_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_ronna_meter_100, main_access.inner_scale_ronna_meter_100);
                            let state_scale_quetta_meter_1 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_quetta_meter_1, main_access.inner_scale_quetta_meter_1);
                            let state_scale_quetta_meter_10 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_quetta_meter_10, main_access.inner_scale_quetta_meter_10);
                            let state_scale_quetta_meter_100 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_quetta_meter_100, main_access.inner_scale_quetta_meter_100);
                            let state_scale_quetta_meter_1000 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_quetta_meter_1000, main_access.inner_scale_quetta_meter_1000);
                            let state_scale_quetta_meter_10000 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_quetta_meter_10000, main_access.inner_scale_quetta_meter_10000);
                            let state_scale_quetta_meter_100000 = unload_and_wait_stage_setup_ecs_while(input.inner_scale_quetta_meter_100000, main_access.inner_scale_quetta_meter_100000);
                            
                            State {
                                inner_scale_quecto_meter_000001: Some(state_scale_quecto_meter_000001),
                                inner_scale_quecto_meter_00001: Some(state_scale_quecto_meter_00001),
                                inner_scale_quecto_meter_0001: Some(state_scale_quecto_meter_0001),
                                inner_scale_quecto_meter_001: Some(state_scale_quecto_meter_001),
                                inner_scale_quecto_meter_01: Some(state_scale_quecto_meter_01),
                                inner_scale_quecto_meter_1: Some(state_scale_quecto_meter_1),
                                inner_scale_quecto_meter_10: Some(state_scale_quecto_meter_10),
                                inner_scale_quecto_meter_100: Some(state_scale_quecto_meter_100),
                                inner_scale_ronto_meter_1: Some(state_scale_ronto_meter_1),
                                inner_scale_ronto_meter_10: Some(state_scale_ronto_meter_10),
                                inner_scale_ronto_meter_100: Some(state_scale_ronto_meter_100),
                                inner_scale_yocto_meter_1: Some(state_scale_yocto_meter_1),
                                inner_scale_yocto_meter_10: Some(state_scale_yocto_meter_10),
                                inner_scale_yocto_meter_100: Some(state_scale_yocto_meter_100),
                                inner_scale_zepto_meter_1: Some(state_scale_zepto_meter_1),
                                inner_scale_zepto_meter_10: Some(state_scale_zepto_meter_10),
                                inner_scale_zepto_meter_100: Some(state_scale_zepto_meter_100),
                                inner_scale_atto_meter_1: Some(state_scale_atto_meter_1),
                                inner_scale_atto_meter_10: Some(state_scale_atto_meter_10),
                                inner_scale_atto_meter_100: Some(state_scale_atto_meter_100),
                                inner_scale_femto_meter_1: Some(state_scale_femto_meter_1),
                                inner_scale_femto_meter_10: Some(state_scale_femto_meter_10),
                                inner_scale_femto_meter_100: Some(state_scale_femto_meter_100),
                                inner_scale_pico_meter_1: Some(state_scale_pico_meter_1),
                                inner_scale_pico_meter_10: Some(state_scale_pico_meter_10),
                                inner_scale_pico_meter_100: Some(state_scale_pico_meter_100),
                                inner_scale_nano_meter_1: Some(state_scale_nano_meter_1),
                                inner_scale_nano_meter_10: Some(state_scale_nano_meter_10),
                                inner_scale_nano_meter_100: Some(state_scale_nano_meter_100),
                                inner_scale_micro_meter_1: Some(state_scale_micro_meter_1),
                                inner_scale_micro_meter_10: Some(state_scale_micro_meter_10),
                                inner_scale_micro_meter_100: Some(state_scale_micro_meter_100),
                                inner_scale_milli_meter_1: Some(state_scale_milli_meter_1),
                                inner_scale_milli_meter_10: Some(state_scale_milli_meter_10),
                                inner_scale_milli_meter_100: Some(state_scale_milli_meter_100),
                                inner_scale_meter_1: Some(state_scale_meter_1),
                                inner_scale_meter_10: Some(state_scale_meter_10),
                                inner_scale_meter_100: Some(state_scale_meter_100),
                                inner_scale_kilo_meter_1: Some(state_scale_kilo_meter_1),
                                inner_scale_kilo_meter_10: Some(state_scale_kilo_meter_10),
                                inner_scale_kilo_meter_100: Some(state_scale_kilo_meter_100),
                                inner_scale_mega_meter_1: Some(state_scale_mega_meter_1),
                                inner_scale_mega_meter_10: Some(state_scale_mega_meter_10),
                                inner_scale_mega_meter_100: Some(state_scale_mega_meter_100),
                                inner_scale_giga_meter_1: Some(state_scale_giga_meter_1),
                                inner_scale_giga_meter_10: Some(state_scale_giga_meter_10),
                                inner_scale_giga_meter_100: Some(state_scale_giga_meter_100),
                                inner_scale_tera_meter_1: Some(state_scale_tera_meter_1),
                                inner_scale_tera_meter_10: Some(state_scale_tera_meter_10),
                                inner_scale_tera_meter_100: Some(state_scale_tera_meter_100),
                                inner_scale_peta_meter_1: Some(state_scale_peta_meter_1),
                                inner_scale_peta_meter_10: Some(state_scale_peta_meter_10),
                                inner_scale_peta_meter_100: Some(state_scale_peta_meter_100),
                                inner_scale_exa_meter_1: Some(state_scale_exa_meter_1),
                                inner_scale_exa_meter_10: Some(state_scale_exa_meter_10),
                                inner_scale_exa_meter_100: Some(state_scale_exa_meter_100),
                                inner_scale_zetta_meter_1: Some(state_scale_zetta_meter_1),
                                inner_scale_zetta_meter_10: Some(state_scale_zetta_meter_10),
                                inner_scale_zetta_meter_100: Some(state_scale_zetta_meter_100),
                                inner_scale_yotta_meter_1: Some(state_scale_yotta_meter_1),
                                inner_scale_yotta_meter_10: Some(state_scale_yotta_meter_10),
                                inner_scale_yotta_meter_100: Some(state_scale_yotta_meter_100),
                                inner_scale_ronna_meter_1: Some(state_scale_ronna_meter_1),
                                inner_scale_ronna_meter_10: Some(state_scale_ronna_meter_10),
                                inner_scale_ronna_meter_100: Some(state_scale_ronna_meter_100),
                                inner_scale_quetta_meter_1: Some(state_scale_quetta_meter_1),
                                inner_scale_quetta_meter_10: Some(state_scale_quetta_meter_10),
                                inner_scale_quetta_meter_100: Some(state_scale_quetta_meter_100),
                                inner_scale_quetta_meter_1000: Some(state_scale_quetta_meter_1000),
                                inner_scale_quetta_meter_10000: Some(state_scale_quetta_meter_10000),
                                inner_scale_quetta_meter_100000: Some(state_scale_quetta_meter_100000),
                            }
                        }

                        fn RunEcsWhile |state, main_access| -> Outcome<State, ()> {
                            let outcome_scale_quecto_meter_000001 = if let Some(state) = state.inner_scale_quecto_meter_000001 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quecto_meter_000001))
                            } else { None };
                            let outcome_scale_quecto_meter_00001 = if let Some(state) = state.inner_scale_quecto_meter_00001 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quecto_meter_00001))
                            } else { None };
                            let outcome_scale_quecto_meter_0001 = if let Some(state) = state.inner_scale_quecto_meter_0001 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quecto_meter_0001))
                            } else { None };
                            let outcome_scale_quecto_meter_001 = if let Some(state) = state.inner_scale_quecto_meter_001 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quecto_meter_001))
                            } else { None };
                            let outcome_scale_quecto_meter_01 = if let Some(state) = state.inner_scale_quecto_meter_01 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quecto_meter_01))
                            } else { None };
                            let outcome_scale_quecto_meter_1 = if let Some(state) = state.inner_scale_quecto_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quecto_meter_1))
                            } else { None };
                            let outcome_scale_quecto_meter_10 = if let Some(state) = state.inner_scale_quecto_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quecto_meter_10))
                            } else { None };
                            let outcome_scale_quecto_meter_100 = if let Some(state) = state.inner_scale_quecto_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quecto_meter_100))
                            } else { None };
                            let outcome_scale_ronto_meter_1 = if let Some(state) = state.inner_scale_ronto_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_ronto_meter_1))
                            } else { None };
                            let outcome_scale_ronto_meter_10 = if let Some(state) = state.inner_scale_ronto_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_ronto_meter_10))
                            } else { None };
                            let outcome_scale_ronto_meter_100 = if let Some(state) = state.inner_scale_ronto_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_ronto_meter_100))
                            } else { None };
                            let outcome_scale_yocto_meter_1 = if let Some(state) = state.inner_scale_yocto_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_yocto_meter_1))
                            } else { None };
                            let outcome_scale_yocto_meter_10 = if let Some(state) = state.inner_scale_yocto_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_yocto_meter_10))
                            } else { None };
                            let outcome_scale_yocto_meter_100 = if let Some(state) = state.inner_scale_yocto_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_yocto_meter_100))
                            } else { None };
                            let outcome_scale_zepto_meter_1 = if let Some(state) = state.inner_scale_zepto_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_zepto_meter_1))
                            } else { None };
                            let outcome_scale_zepto_meter_10 = if let Some(state) = state.inner_scale_zepto_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_zepto_meter_10))
                            } else { None };
                            let outcome_scale_zepto_meter_100 = if let Some(state) = state.inner_scale_zepto_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_zepto_meter_100))
                            } else { None };
                            let outcome_scale_atto_meter_1 = if let Some(state) = state.inner_scale_atto_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_atto_meter_1))
                            } else { None };
                            let outcome_scale_atto_meter_10 = if let Some(state) = state.inner_scale_atto_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_atto_meter_10))
                            } else { None };
                            let outcome_scale_atto_meter_100 = if let Some(state) = state.inner_scale_atto_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_atto_meter_100))
                            } else { None };
                            let outcome_scale_femto_meter_1 = if let Some(state) = state.inner_scale_femto_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_femto_meter_1))
                            } else { None };
                            let outcome_scale_femto_meter_10 = if let Some(state) = state.inner_scale_femto_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_femto_meter_10))
                            } else { None };
                            let outcome_scale_femto_meter_100 = if let Some(state) = state.inner_scale_femto_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_femto_meter_100))
                            } else { None };
                            let outcome_scale_pico_meter_1 = if let Some(state) = state.inner_scale_pico_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_pico_meter_1))
                            } else { None };
                            let outcome_scale_pico_meter_10 = if let Some(state) = state.inner_scale_pico_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_pico_meter_10))
                            } else { None };
                            let outcome_scale_pico_meter_100 = if let Some(state) = state.inner_scale_pico_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_pico_meter_100))
                            } else { None };
                            let outcome_scale_nano_meter_1 = if let Some(state) = state.inner_scale_nano_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_nano_meter_1))
                            } else { None };
                            let outcome_scale_nano_meter_10 = if let Some(state) = state.inner_scale_nano_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_nano_meter_10))
                            } else { None };
                            let outcome_scale_nano_meter_100 = if let Some(state) = state.inner_scale_nano_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_nano_meter_100))
                            } else { None };
                            let outcome_scale_micro_meter_1 = if let Some(state) = state.inner_scale_micro_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_micro_meter_1))
                            } else { None };
                            let outcome_scale_micro_meter_10 = if let Some(state) = state.inner_scale_micro_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_micro_meter_10))
                            } else { None };
                            let outcome_scale_micro_meter_100 = if let Some(state) = state.inner_scale_micro_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_micro_meter_100))
                            } else { None };
                            let outcome_scale_milli_meter_1 = if let Some(state) = state.inner_scale_milli_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_milli_meter_1))
                            } else { None };
                            let outcome_scale_milli_meter_10 = if let Some(state) = state.inner_scale_milli_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_milli_meter_10))
                            } else { None };
                            let outcome_scale_milli_meter_100 = if let Some(state) = state.inner_scale_milli_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_milli_meter_100))
                            } else { None };
                            let outcome_scale_meter_1 = if let Some(state) = state.inner_scale_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_meter_1))
                            } else { None };
                            let outcome_scale_meter_10 = if let Some(state) = state.inner_scale_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_meter_10))
                            } else { None };
                            let outcome_scale_meter_100 = if let Some(state) = state.inner_scale_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_meter_100))
                            } else { None };
                            let outcome_scale_kilo_meter_1 = if let Some(state) = state.inner_scale_kilo_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_kilo_meter_1))
                            } else { None };
                            let outcome_scale_kilo_meter_10 = if let Some(state) = state.inner_scale_kilo_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_kilo_meter_10))
                            } else { None };
                            let outcome_scale_kilo_meter_100 = if let Some(state) = state.inner_scale_kilo_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_kilo_meter_100))
                            } else { None };
                            let outcome_scale_mega_meter_1 = if let Some(state) = state.inner_scale_mega_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_mega_meter_1))
                            } else { None };
                            let outcome_scale_mega_meter_10 = if let Some(state) = state.inner_scale_mega_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_mega_meter_10))
                            } else { None };
                            let outcome_scale_mega_meter_100 = if let Some(state) = state.inner_scale_mega_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_mega_meter_100))
                            } else { None };
                            let outcome_scale_giga_meter_1 = if let Some(state) = state.inner_scale_giga_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_giga_meter_1))
                            } else { None };
                            let outcome_scale_giga_meter_10 = if let Some(state) = state.inner_scale_giga_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_giga_meter_10))
                            } else { None };
                            let outcome_scale_giga_meter_100 = if let Some(state) = state.inner_scale_giga_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_giga_meter_100))
                            } else { None };
                            let outcome_scale_tera_meter_1 = if let Some(state) = state.inner_scale_tera_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_tera_meter_1))
                            } else { None };
                            let outcome_scale_tera_meter_10 = if let Some(state) = state.inner_scale_tera_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_tera_meter_10))
                            } else { None };
                            let outcome_scale_tera_meter_100 = if let Some(state) = state.inner_scale_tera_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_tera_meter_100))
                            } else { None };
                            let outcome_scale_peta_meter_1 = if let Some(state) = state.inner_scale_peta_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_peta_meter_1))
                            } else { None };
                            let outcome_scale_peta_meter_10 = if let Some(state) = state.inner_scale_peta_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_peta_meter_10))
                            } else { None };
                            let outcome_scale_peta_meter_100 = if let Some(state) = state.inner_scale_peta_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_peta_meter_100))
                            } else { None };
                            let outcome_scale_exa_meter_1 = if let Some(state) = state.inner_scale_exa_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_exa_meter_1))
                            } else { None };
                            let outcome_scale_exa_meter_10 = if let Some(state) = state.inner_scale_exa_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_exa_meter_10))
                            } else { None };
                            let outcome_scale_exa_meter_100 = if let Some(state) = state.inner_scale_exa_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_exa_meter_100))
                            } else { None };
                            let outcome_scale_zetta_meter_1 = if let Some(state) = state.inner_scale_zetta_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_zetta_meter_1))
                            } else { None };
                            let outcome_scale_zetta_meter_10 = if let Some(state) = state.inner_scale_zetta_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_zetta_meter_10))
                            } else { None };
                            let outcome_scale_zetta_meter_100 = if let Some(state) = state.inner_scale_zetta_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_zetta_meter_100))
                            } else { None };
                            let outcome_scale_yotta_meter_1 = if let Some(state) = state.inner_scale_yotta_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_yotta_meter_1))
                            } else { None };
                            let outcome_scale_yotta_meter_10 = if let Some(state) = state.inner_scale_yotta_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_yotta_meter_10))
                            } else { None };
                            let outcome_scale_yotta_meter_100 = if let Some(state) = state.inner_scale_yotta_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_yotta_meter_100))
                            } else { None };
                            let outcome_scale_ronna_meter_1 = if let Some(state) = state.inner_scale_ronna_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_ronna_meter_1))
                            } else { None };
                            let outcome_scale_ronna_meter_10 = if let Some(state) = state.inner_scale_ronna_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_ronna_meter_10))
                            } else { None };
                            let outcome_scale_ronna_meter_100 = if let Some(state) = state.inner_scale_ronna_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_ronna_meter_100))
                            } else { None };
                            let outcome_scale_quetta_meter_1 = if let Some(state) = state.inner_scale_quetta_meter_1 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quetta_meter_1))
                            } else { None };
                            let outcome_scale_quetta_meter_10 = if let Some(state) = state.inner_scale_quetta_meter_10 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quetta_meter_10))
                            } else { None };
                            let outcome_scale_quetta_meter_100 = if let Some(state) = state.inner_scale_quetta_meter_100 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quetta_meter_100))
                            } else { None };
                            let outcome_scale_quetta_meter_1000 = if let Some(state) = state.inner_scale_quetta_meter_1000 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quetta_meter_1000))
                            } else { None };
                            let outcome_scale_quetta_meter_10000 = if let Some(state) = state.inner_scale_quetta_meter_10000 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quetta_meter_10000))
                            } else { None };
                            let outcome_scale_quetta_meter_100000 = if let Some(state) = state.inner_scale_quetta_meter_100000 {
                                Some(unload_and_wait_stage_run_ecs_while(state, main_access.inner_scale_quetta_meter_100000))
                            } else { None };

                            if outcome_scale_quecto_meter_000001.is_none() 
                                && outcome_scale_quecto_meter_00001.is_none() 
                                && outcome_scale_quecto_meter_0001.is_none() 
                                && outcome_scale_quecto_meter_001.is_none() 
                                && outcome_scale_quecto_meter_01.is_none() 
                                && outcome_scale_quecto_meter_1.is_none() 
                                && outcome_scale_quecto_meter_10.is_none() 
                                && outcome_scale_quecto_meter_100.is_none() 
                                && outcome_scale_ronto_meter_1.is_none() 
                                && outcome_scale_ronto_meter_10.is_none() 
                                && outcome_scale_ronto_meter_100.is_none() 
                                && outcome_scale_yocto_meter_1.is_none() 
                                && outcome_scale_yocto_meter_10.is_none() 
                                && outcome_scale_yocto_meter_100.is_none() 
                                && outcome_scale_zepto_meter_1.is_none() 
                                && outcome_scale_zepto_meter_10.is_none() 
                                && outcome_scale_zepto_meter_100.is_none() 
                                && outcome_scale_atto_meter_1.is_none() 
                                && outcome_scale_atto_meter_10.is_none() 
                                && outcome_scale_atto_meter_100.is_none() 
                                && outcome_scale_femto_meter_1.is_none() 
                                && outcome_scale_femto_meter_10.is_none() 
                                && outcome_scale_femto_meter_100.is_none() 
                                && outcome_scale_pico_meter_1.is_none() 
                                && outcome_scale_pico_meter_10.is_none() 
                                && outcome_scale_pico_meter_100.is_none() 
                                && outcome_scale_nano_meter_1.is_none() 
                                && outcome_scale_nano_meter_10.is_none() 
                                && outcome_scale_nano_meter_100.is_none() 
                                && outcome_scale_micro_meter_1.is_none() 
                                && outcome_scale_micro_meter_10.is_none() 
                                && outcome_scale_micro_meter_100.is_none() 
                                && outcome_scale_milli_meter_1.is_none() 
                                && outcome_scale_milli_meter_10.is_none() 
                                && outcome_scale_milli_meter_100.is_none() 
                                && outcome_scale_meter_1.is_none() 
                                && outcome_scale_meter_10.is_none() 
                                && outcome_scale_meter_100.is_none() 
                                && outcome_scale_kilo_meter_1.is_none() 
                                && outcome_scale_kilo_meter_10.is_none() 
                                && outcome_scale_kilo_meter_100.is_none() 
                                && outcome_scale_mega_meter_1.is_none() 
                                && outcome_scale_mega_meter_10.is_none() 
                                && outcome_scale_mega_meter_100.is_none() 
                                && outcome_scale_giga_meter_1.is_none() 
                                && outcome_scale_giga_meter_10.is_none() 
                                && outcome_scale_giga_meter_100.is_none() 
                                && outcome_scale_tera_meter_1.is_none() 
                                && outcome_scale_tera_meter_10.is_none() 
                                && outcome_scale_tera_meter_100.is_none() 
                                && outcome_scale_peta_meter_1.is_none() 
                                && outcome_scale_peta_meter_10.is_none() 
                                && outcome_scale_peta_meter_100.is_none() 
                                && outcome_scale_exa_meter_1.is_none() 
                                && outcome_scale_exa_meter_10.is_none() 
                                && outcome_scale_exa_meter_100.is_none() 
                                && outcome_scale_zetta_meter_1.is_none() 
                                && outcome_scale_zetta_meter_10.is_none() 
                                && outcome_scale_zetta_meter_100.is_none() 
                                && outcome_scale_yotta_meter_1.is_none() 
                                && outcome_scale_yotta_meter_10.is_none() 
                                && outcome_scale_yotta_meter_100.is_none() 
                                && outcome_scale_ronna_meter_1.is_none() 
                                && outcome_scale_ronna_meter_10.is_none() 
                                && outcome_scale_ronna_meter_100.is_none() 
                                && outcome_scale_quetta_meter_1.is_none() 
                                && outcome_scale_quetta_meter_10.is_none() 
                                && outcome_scale_quetta_meter_100.is_none() 
                                && outcome_scale_quetta_meter_1000.is_none() 
                                && outcome_scale_quetta_meter_10000.is_none() 
                                && outcome_scale_quetta_meter_100000.is_none() 
                            {
                                return Done(());
                            }

                            let outcome_scale_quecto_meter_000001 = match outcome_scale_quecto_meter_000001 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quecto_meter_00001 = match outcome_scale_quecto_meter_00001 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quecto_meter_0001 = match outcome_scale_quecto_meter_0001 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quecto_meter_001 = match outcome_scale_quecto_meter_001 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quecto_meter_01 = match outcome_scale_quecto_meter_01 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quecto_meter_1 = match outcome_scale_quecto_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quecto_meter_10 = match outcome_scale_quecto_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quecto_meter_100 = match outcome_scale_quecto_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_ronto_meter_1 = match outcome_scale_ronto_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_ronto_meter_10 = match outcome_scale_ronto_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_ronto_meter_100 = match outcome_scale_ronto_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_yocto_meter_1 = match outcome_scale_yocto_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_yocto_meter_10 = match outcome_scale_yocto_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_yocto_meter_100 = match outcome_scale_yocto_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_zepto_meter_1 = match outcome_scale_zepto_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_zepto_meter_10 = match outcome_scale_zepto_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_zepto_meter_100 = match outcome_scale_zepto_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_atto_meter_1 = match outcome_scale_atto_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_atto_meter_10 = match outcome_scale_atto_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_atto_meter_100 = match outcome_scale_atto_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_femto_meter_1 = match outcome_scale_femto_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_femto_meter_10 = match outcome_scale_femto_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_femto_meter_100 = match outcome_scale_femto_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_pico_meter_1 = match outcome_scale_pico_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_pico_meter_10 = match outcome_scale_pico_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_pico_meter_100 = match outcome_scale_pico_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_nano_meter_1 = match outcome_scale_nano_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_nano_meter_10 = match outcome_scale_nano_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_nano_meter_100 = match outcome_scale_nano_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_micro_meter_1 = match outcome_scale_micro_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_micro_meter_10 = match outcome_scale_micro_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_micro_meter_100 = match outcome_scale_micro_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_milli_meter_1 = match outcome_scale_milli_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_milli_meter_10 = match outcome_scale_milli_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_milli_meter_100 = match outcome_scale_milli_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_meter_1 = match outcome_scale_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_meter_10 = match outcome_scale_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_meter_100 = match outcome_scale_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_kilo_meter_1 = match outcome_scale_kilo_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_kilo_meter_10 = match outcome_scale_kilo_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_kilo_meter_100 = match outcome_scale_kilo_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_mega_meter_1 = match outcome_scale_mega_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_mega_meter_10 = match outcome_scale_mega_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_mega_meter_100 = match outcome_scale_mega_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_giga_meter_1 = match outcome_scale_giga_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_giga_meter_10 = match outcome_scale_giga_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_giga_meter_100 = match outcome_scale_giga_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_tera_meter_1 = match outcome_scale_tera_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_tera_meter_10 = match outcome_scale_tera_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_tera_meter_100 = match outcome_scale_tera_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_peta_meter_1 = match outcome_scale_peta_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_peta_meter_10 = match outcome_scale_peta_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_peta_meter_100 = match outcome_scale_peta_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_exa_meter_1 = match outcome_scale_exa_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_exa_meter_10 = match outcome_scale_exa_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_exa_meter_100 = match outcome_scale_exa_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_zetta_meter_1 = match outcome_scale_zetta_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_zetta_meter_10 = match outcome_scale_zetta_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_zetta_meter_100 = match outcome_scale_zetta_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_yotta_meter_1 = match outcome_scale_yotta_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_yotta_meter_10 = match outcome_scale_yotta_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_yotta_meter_100 = match outcome_scale_yotta_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_ronna_meter_1 = match outcome_scale_ronna_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_ronna_meter_10 = match outcome_scale_ronna_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_ronna_meter_100 = match outcome_scale_ronna_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quetta_meter_1 = match outcome_scale_quetta_meter_1 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quetta_meter_10 = match outcome_scale_quetta_meter_10 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quetta_meter_100 = match outcome_scale_quetta_meter_100 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quetta_meter_1000 = match outcome_scale_quetta_meter_1000 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quetta_meter_10000 = match outcome_scale_quetta_meter_10000 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };
                            let outcome_scale_quetta_meter_100000 = match outcome_scale_quetta_meter_100000 {
                                Some(Outcome::Wait(state)) => Some(state),
                                Some(Outcome::Done(_)) => None,
                                None => None,
                            };

                            Wait(State {
                                inner_scale_quecto_meter_000001: outcome_scale_quecto_meter_000001,
                                inner_scale_quecto_meter_00001: outcome_scale_quecto_meter_00001,
                                inner_scale_quecto_meter_0001: outcome_scale_quecto_meter_0001,
                                inner_scale_quecto_meter_001: outcome_scale_quecto_meter_001,
                                inner_scale_quecto_meter_01: outcome_scale_quecto_meter_01,
                                inner_scale_quecto_meter_1: outcome_scale_quecto_meter_1,
                                inner_scale_quecto_meter_10: outcome_scale_quecto_meter_10,
                                inner_scale_quecto_meter_100: outcome_scale_quecto_meter_100,
                                inner_scale_ronto_meter_1: outcome_scale_ronto_meter_1,
                                inner_scale_ronto_meter_10: outcome_scale_ronto_meter_10,
                                inner_scale_ronto_meter_100: outcome_scale_ronto_meter_100,
                                inner_scale_yocto_meter_1: outcome_scale_yocto_meter_1,
                                inner_scale_yocto_meter_10: outcome_scale_yocto_meter_10,
                                inner_scale_yocto_meter_100: outcome_scale_yocto_meter_100,
                                inner_scale_zepto_meter_1: outcome_scale_zepto_meter_1,
                                inner_scale_zepto_meter_10: outcome_scale_zepto_meter_10,
                                inner_scale_zepto_meter_100: outcome_scale_zepto_meter_100,
                                inner_scale_atto_meter_1: outcome_scale_atto_meter_1,
                                inner_scale_atto_meter_10: outcome_scale_atto_meter_10,
                                inner_scale_atto_meter_100: outcome_scale_atto_meter_100,
                                inner_scale_femto_meter_1: outcome_scale_femto_meter_1,
                                inner_scale_femto_meter_10: outcome_scale_femto_meter_10,
                                inner_scale_femto_meter_100: outcome_scale_femto_meter_100,
                                inner_scale_pico_meter_1: outcome_scale_pico_meter_1,
                                inner_scale_pico_meter_10: outcome_scale_pico_meter_10,
                                inner_scale_pico_meter_100: outcome_scale_pico_meter_100,
                                inner_scale_nano_meter_1: outcome_scale_nano_meter_1,
                                inner_scale_nano_meter_10: outcome_scale_nano_meter_10,
                                inner_scale_nano_meter_100: outcome_scale_nano_meter_100,
                                inner_scale_micro_meter_1: outcome_scale_micro_meter_1,
                                inner_scale_micro_meter_10: outcome_scale_micro_meter_10,
                                inner_scale_micro_meter_100: outcome_scale_micro_meter_100,
                                inner_scale_milli_meter_1: outcome_scale_milli_meter_1,
                                inner_scale_milli_meter_10: outcome_scale_milli_meter_10,
                                inner_scale_milli_meter_100: outcome_scale_milli_meter_100,
                                inner_scale_meter_1: outcome_scale_meter_1,
                                inner_scale_meter_10: outcome_scale_meter_10,
                                inner_scale_meter_100: outcome_scale_meter_100,
                                inner_scale_kilo_meter_1: outcome_scale_kilo_meter_1,
                                inner_scale_kilo_meter_10: outcome_scale_kilo_meter_10,
                                inner_scale_kilo_meter_100: outcome_scale_kilo_meter_100,
                                inner_scale_mega_meter_1: outcome_scale_mega_meter_1,
                                inner_scale_mega_meter_10: outcome_scale_mega_meter_10,
                                inner_scale_mega_meter_100: outcome_scale_mega_meter_100,
                                inner_scale_giga_meter_1: outcome_scale_giga_meter_1,
                                inner_scale_giga_meter_10: outcome_scale_giga_meter_10,
                                inner_scale_giga_meter_100: outcome_scale_giga_meter_100,
                                inner_scale_tera_meter_1: outcome_scale_tera_meter_1,
                                inner_scale_tera_meter_10: outcome_scale_tera_meter_10,
                                inner_scale_tera_meter_100: outcome_scale_tera_meter_100,
                                inner_scale_peta_meter_1: outcome_scale_peta_meter_1,
                                inner_scale_peta_meter_10: outcome_scale_peta_meter_10,
                                inner_scale_peta_meter_100: outcome_scale_peta_meter_100,
                                inner_scale_exa_meter_1: outcome_scale_exa_meter_1,
                                inner_scale_exa_meter_10: outcome_scale_exa_meter_10,
                                inner_scale_exa_meter_100: outcome_scale_exa_meter_100,
                                inner_scale_zetta_meter_1: outcome_scale_zetta_meter_1,
                                inner_scale_zetta_meter_10: outcome_scale_zetta_meter_10,
                                inner_scale_zetta_meter_100: outcome_scale_zetta_meter_100,
                                inner_scale_yotta_meter_1: outcome_scale_yotta_meter_1,
                                inner_scale_yotta_meter_10: outcome_scale_yotta_meter_10,
                                inner_scale_yotta_meter_100: outcome_scale_yotta_meter_100,
                                inner_scale_ronna_meter_1: outcome_scale_ronna_meter_1,
                                inner_scale_ronna_meter_10: outcome_scale_ronna_meter_10,
                                inner_scale_ronna_meter_100: outcome_scale_ronna_meter_100,
                                inner_scale_quetta_meter_1: outcome_scale_quetta_meter_1,
                                inner_scale_quetta_meter_10: outcome_scale_quetta_meter_10,
                                inner_scale_quetta_meter_100: outcome_scale_quetta_meter_100,
                                inner_scale_quetta_meter_1000: outcome_scale_quetta_meter_1000,
                                inner_scale_quetta_meter_10000: outcome_scale_quetta_meter_10000,
                                inner_scale_quetta_meter_100000: outcome_scale_quetta_meter_100000,
                            })
                        }
                    ]
                }
            ]
        }
    ]
}
