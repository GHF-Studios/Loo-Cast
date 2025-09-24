use bevy::prelude::*;
use core_mod_macros::{composite_workflow, composite_workflow_return};

use crate::usf::scale::*;
use crate::{chunk::types::ChunkOwnerId, chunk_loader::components::ChunkLoader, workflow::functions::handle_composite_workflow_return_later};

use super::types::RemovedChunkLoaderObservation;
use super::resources::RemovedChunkLoaderObservationQueue;

#[tracing::instrument(skip_all)]
pub(crate) fn observe_on_remove_chunk_loader<S: Scale>(
    trigger: Trigger<OnRemove, ChunkLoader<S>>,
    mut queue: ResMut<RemovedChunkLoaderObservationQueue>,
) {
    let loader_entity = trigger.target();
    queue.0.insert(RemovedChunkLoaderObservation { entity: loader_entity, scale: S::SCALE_FACTOR_EXPONENT });
}


// TODO: MAJOR: This silently drops observed chunk loader removals if one is already in-progress composite-workflow-wise, so for now:
// Concurrent chunk loader removals are unsound!
#[tracing::instrument(skip_all)]
pub(crate) fn on_remove_chunk_loader_observation_queue_processing_system(
    mut queue: ResMut<RemovedChunkLoaderObservationQueue>,
    chunk_loader_query_scale_quecto_meter_000001: Query<(&Transform, &ChunkLoader<ScaleQuectoMeter000001>)>,
    chunk_loader_query_scale_quecto_meter_00001: Query<(&Transform, &ChunkLoader<ScaleQuectoMeter00001>)>,
    chunk_loader_query_scale_quecto_meter_0001: Query<(&Transform, &ChunkLoader<ScaleQuectoMeter0001>)>,
    chunk_loader_query_scale_quecto_meter_001: Query<(&Transform, &ChunkLoader<ScaleQuectoMeter001>)>,
    chunk_loader_query_scale_quecto_meter_01: Query<(&Transform, &ChunkLoader<ScaleQuectoMeter01>)>,
    chunk_loader_query_scale_quecto_meter_1: Query<(&Transform, &ChunkLoader<ScaleQuectoMeter1>)>,
    chunk_loader_query_scale_quecto_meter_10: Query<(&Transform, &ChunkLoader<ScaleQuectoMeter10>)>,
    chunk_loader_query_scale_quecto_meter_100: Query<(&Transform, &ChunkLoader<ScaleQuectoMeter100>)>,
    chunk_loader_query_scale_ronto_meter_1: Query<(&Transform, &ChunkLoader<ScaleRontoMeter1>)>,
    chunk_loader_query_scale_ronto_meter_10: Query<(&Transform, &ChunkLoader<ScaleRontoMeter10>)>,
    chunk_loader_query_scale_ronto_meter_100: Query<(&Transform, &ChunkLoader<ScaleRontoMeter100>)>,
    chunk_loader_query_scale_yocto_meter_1: Query<(&Transform, &ChunkLoader<ScaleYoctoMeter1>)>,
    chunk_loader_query_scale_yocto_meter_10: Query<(&Transform, &ChunkLoader<ScaleYoctoMeter10>)>,
    chunk_loader_query_scale_yocto_meter_100: Query<(&Transform, &ChunkLoader<ScaleYoctoMeter100>)>,
    chunk_loader_query_scale_zepto_meter_1: Query<(&Transform, &ChunkLoader<ScaleZeptoMeter1>)>,
    chunk_loader_query_scale_zepto_meter_10: Query<(&Transform, &ChunkLoader<ScaleZeptoMeter10>)>,
    chunk_loader_query_scale_zepto_meter_100: Query<(&Transform, &ChunkLoader<ScaleZeptoMeter100>)>,
    chunk_loader_query_scale_atto_meter_1: Query<(&Transform, &ChunkLoader<ScaleAttoMeter1>)>,
    chunk_loader_query_scale_atto_meter_10: Query<(&Transform, &ChunkLoader<ScaleAttoMeter10>)>,
    chunk_loader_query_scale_atto_meter_100: Query<(&Transform, &ChunkLoader<ScaleAttoMeter100>)>,
    chunk_loader_query_scale_femto_meter_1: Query<(&Transform, &ChunkLoader<ScaleFemtoMeter1>)>,
    chunk_loader_query_scale_femto_meter_10: Query<(&Transform, &ChunkLoader<ScaleFemtoMeter10>)>,
    chunk_loader_query_scale_femto_meter_100: Query<(&Transform, &ChunkLoader<ScaleFemtoMeter100>)>,
    chunk_loader_query_scale_pico_meter_1: Query<(&Transform, &ChunkLoader<ScalePicoMeter1>)>,
    chunk_loader_query_scale_pico_meter_10: Query<(&Transform, &ChunkLoader<ScalePicoMeter10>)>,
    chunk_loader_query_scale_pico_meter_100: Query<(&Transform, &ChunkLoader<ScalePicoMeter100>)>,
    chunk_loader_query_scale_nano_meter_1: Query<(&Transform, &ChunkLoader<ScaleNanoMeter1>)>,
    chunk_loader_query_scale_nano_meter_10: Query<(&Transform, &ChunkLoader<ScaleNanoMeter10>)>,
    chunk_loader_query_scale_nano_meter_100: Query<(&Transform, &ChunkLoader<ScaleNanoMeter100>)>,
    chunk_loader_query_scale_micro_meter_1: Query<(&Transform, &ChunkLoader<ScaleMicroMeter1>)>,
    chunk_loader_query_scale_micro_meter_10: Query<(&Transform, &ChunkLoader<ScaleMicroMeter10>)>,
    chunk_loader_query_scale_micro_meter_100: Query<(&Transform, &ChunkLoader<ScaleMicroMeter100>)>,
    chunk_loader_query_scale_milli_meter_1: Query<(&Transform, &ChunkLoader<ScaleMilliMeter1>)>,
    chunk_loader_query_scale_milli_meter_10: Query<(&Transform, &ChunkLoader<ScaleMilliMeter10>)>,
    chunk_loader_query_scale_milli_meter_100: Query<(&Transform, &ChunkLoader<ScaleMilliMeter100>)>,
    chunk_loader_query_scale_meter_1: Query<(&Transform, &ChunkLoader<ScaleMeter1>)>,
    chunk_loader_query_scale_meter_10: Query<(&Transform, &ChunkLoader<ScaleMeter10>)>,
    chunk_loader_query_scale_meter_100: Query<(&Transform, &ChunkLoader<ScaleMeter100>)>,
    chunk_loader_query_scale_kilo_meter_1: Query<(&Transform, &ChunkLoader<ScaleKiloMeter1>)>,
    chunk_loader_query_scale_kilo_meter_10: Query<(&Transform, &ChunkLoader<ScaleKiloMeter10>)>,
    chunk_loader_query_scale_kilo_meter_100: Query<(&Transform, &ChunkLoader<ScaleKiloMeter100>)>,
    chunk_loader_query_scale_mega_meter_1: Query<(&Transform, &ChunkLoader<ScaleMegaMeter1>)>,
    chunk_loader_query_scale_mega_meter_10: Query<(&Transform, &ChunkLoader<ScaleMegaMeter10>)>,
    chunk_loader_query_scale_mega_meter_100: Query<(&Transform, &ChunkLoader<ScaleMegaMeter100>)>,
    chunk_loader_query_scale_giga_meter_1: Query<(&Transform, &ChunkLoader<ScaleGigaMeter1>)>,
    chunk_loader_query_scale_giga_meter_10: Query<(&Transform, &ChunkLoader<ScaleGigaMeter10>)>,
    chunk_loader_query_scale_giga_meter_100: Query<(&Transform, &ChunkLoader<ScaleGigaMeter100>)>,
    chunk_loader_query_scale_tera_meter_1: Query<(&Transform, &ChunkLoader<ScaleTeraMeter1>)>,
    chunk_loader_query_scale_tera_meter_10: Query<(&Transform, &ChunkLoader<ScaleTeraMeter10>)>,
    chunk_loader_query_scale_tera_meter_100: Query<(&Transform, &ChunkLoader<ScaleTeraMeter100>)>,
    chunk_loader_query_scale_peta_meter_1: Query<(&Transform, &ChunkLoader<ScalePetaMeter1>)>,
    chunk_loader_query_scale_peta_meter_10: Query<(&Transform, &ChunkLoader<ScalePetaMeter10>)>,
    chunk_loader_query_scale_peta_meter_100: Query<(&Transform, &ChunkLoader<ScalePetaMeter100>)>,
    chunk_loader_query_scale_exa_meter_1: Query<(&Transform, &ChunkLoader<ScaleExaMeter1>)>,
    chunk_loader_query_scale_exa_meter_10: Query<(&Transform, &ChunkLoader<ScaleExaMeter10>)>,
    chunk_loader_query_scale_exa_meter_100: Query<(&Transform, &ChunkLoader<ScaleExaMeter100>)>,
    chunk_loader_query_scale_zetta_meter_1: Query<(&Transform, &ChunkLoader<ScaleZettaMeter1>)>,
    chunk_loader_query_scale_zetta_meter_10: Query<(&Transform, &ChunkLoader<ScaleZettaMeter10>)>,
    chunk_loader_query_scale_zetta_meter_100: Query<(&Transform, &ChunkLoader<ScaleZettaMeter100>)>,
    chunk_loader_query_scale_yotta_meter_1: Query<(&Transform, &ChunkLoader<ScaleYottaMeter1>)>,
    chunk_loader_query_scale_yotta_meter_10: Query<(&Transform, &ChunkLoader<ScaleYottaMeter10>)>,
    chunk_loader_query_scale_yotta_meter_100: Query<(&Transform, &ChunkLoader<ScaleYottaMeter100>)>,
    chunk_loader_query_scale_ronna_meter_1: Query<(&Transform, &ChunkLoader<ScaleRonnaMeter1>)>,
    chunk_loader_query_scale_ronna_meter_10: Query<(&Transform, &ChunkLoader<ScaleRonnaMeter10>)>,
    chunk_loader_query_scale_ronna_meter_100: Query<(&Transform, &ChunkLoader<ScaleRonnaMeter100>)>,
    chunk_loader_query_scale_quetta_meter_1: Query<(&Transform, &ChunkLoader<ScaleQuettaMeter1>)>,
    chunk_loader_query_scale_quetta_meter_10: Query<(&Transform, &ChunkLoader<ScaleQuettaMeter10>)>,
    chunk_loader_query_scale_quetta_meter_100: Query<(&Transform, &ChunkLoader<ScaleQuettaMeter100>)>,
    chunk_loader_query_scale_quetta_meter_1000: Query<(&Transform, &ChunkLoader<ScaleQuettaMeter1000>)>,
    chunk_loader_query_scale_quetta_meter_10000: Query<(&Transform, &ChunkLoader<ScaleQuettaMeter10000>)>,
    chunk_loader_query_scale_quetta_meter_100000: Query<(&Transform, &ChunkLoader<ScaleQuettaMeter100000>)>,
) {
    let removed_owner_id_scale_quecto_meter_000001 = None;
    let removed_owner_id_scale_quecto_meter_00001 = None;
    let removed_owner_id_scale_quecto_meter_0001 = None;
    let removed_owner_id_scale_quecto_meter_001 = None;
    let removed_owner_id_scale_quecto_meter_01 = None;
    let removed_owner_id_scale_quecto_meter_1 = None;
    let removed_owner_id_scale_quecto_meter_10 = None;
    let removed_owner_id_scale_quecto_meter_100 = None;
    let removed_owner_id_scale_ronto_meter_1 = None;
    let removed_owner_id_scale_ronto_meter_10 = None;
    let removed_owner_id_scale_ronto_meter_100 = None;
    let removed_owner_id_scale_yocto_meter_1 = None;
    let removed_owner_id_scale_yocto_meter_10 = None;
    let removed_owner_id_scale_yocto_meter_100 = None;
    let removed_owner_id_scale_zepto_meter_1 = None;
    let removed_owner_id_scale_zepto_meter_10 = None;
    let removed_owner_id_scale_zepto_meter_100 = None;
    let removed_owner_id_scale_atto_meter_1 = None;
    let removed_owner_id_scale_atto_meter_10 = None;
    let removed_owner_id_scale_atto_meter_100 = None;
    let removed_owner_id_scale_femto_meter_1 = None;
    let removed_owner_id_scale_femto_meter_10 = None;
    let removed_owner_id_scale_femto_meter_100 = None;
    let removed_owner_id_scale_pico_meter_1 = None;
    let removed_owner_id_scale_pico_meter_10 = None;
    let removed_owner_id_scale_pico_meter_100 = None;
    let removed_owner_id_scale_nano_meter_1 = None;
    let removed_owner_id_scale_nano_meter_10 = None;
    let removed_owner_id_scale_nano_meter_100 = None;
    let removed_owner_id_scale_micro_meter_1 = None;
    let removed_owner_id_scale_micro_meter_10 = None;
    let removed_owner_id_scale_micro_meter_100 = None;
    let removed_owner_id_scale_milli_meter_1 = None;
    let removed_owner_id_scale_milli_meter_10 = None;
    let removed_owner_id_scale_milli_meter_100 = None;
    let removed_owner_id_scale_meter_1 = None;
    let removed_owner_id_scale_meter_10 = None;
    let removed_owner_id_scale_meter_100 = None;
    let removed_owner_id_scale_kilo_meter_1 = None;
    let removed_owner_id_scale_kilo_meter_10 = None;
    let removed_owner_id_scale_kilo_meter_100 = None;
    let removed_owner_id_scale_mega_meter_1 = None;
    let removed_owner_id_scale_mega_meter_10 = None;
    let removed_owner_id_scale_mega_meter_100 = None;
    let removed_owner_id_scale_giga_meter_1 = None;
    let removed_owner_id_scale_giga_meter_10 = None;
    let removed_owner_id_scale_giga_meter_100 = None;
    let removed_owner_id_scale_tera_meter_1 = None;
    let removed_owner_id_scale_tera_meter_10 = None;
    let removed_owner_id_scale_tera_meter_100 = None;
    let removed_owner_id_scale_peta_meter_1 = None;
    let removed_owner_id_scale_peta_meter_10 = None;
    let removed_owner_id_scale_peta_meter_100 = None;
    let removed_owner_id_scale_exa_meter_1 = None;
    let removed_owner_id_scale_exa_meter_10 = None;
    let removed_owner_id_scale_exa_meter_100 = None;
    let removed_owner_id_scale_zetta_meter_1 = None;
    let removed_owner_id_scale_zetta_meter_10 = None;
    let removed_owner_id_scale_zetta_meter_100 = None;
    let removed_owner_id_scale_yotta_meter_1 = None;
    let removed_owner_id_scale_yotta_meter_10 = None;
    let removed_owner_id_scale_yotta_meter_100 = None;
    let removed_owner_id_scale_ronna_meter_1 = None;
    let removed_owner_id_scale_ronna_meter_10 = None;
    let removed_owner_id_scale_ronna_meter_100 = None;
    let removed_owner_id_scale_quetta_meter_1 = None;
    let removed_owner_id_scale_quetta_meter_10 = None;
    let removed_owner_id_scale_quetta_meter_100 = None;
    let removed_owner_id_scale_quetta_meter_1000 = None;
    let removed_owner_id_scale_quetta_meter_10000 = None;
    let removed_owner_id_scale_quetta_meter_100000 = None;

    for RemovedChunkLoaderObservation { entity: loader_entity, scale } in std::mem::take(&mut queue.0).into_iter() {
        match scale {
            -35 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_quecto_meter_000001.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_quecto_meter_000001 = Some(loader.chunk_owner_id().clone());
            }
            -34 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_quecto_meter_00001.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_quecto_meter_00001 = Some(loader.chunk_owner_id().clone());
            }
            -33 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_quecto_meter_0001.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_quecto_meter_0001 = Some(loader.chunk_owner_id().clone());
            }
            -32 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_quecto_meter_001.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_quecto_meter_001 = Some(loader.chunk_owner_id().clone());
            }
            -31 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_quecto_meter_01.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_quecto_meter_01 = Some(loader.chunk_owner_id().clone());
            }
            -30 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_quecto_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_quecto_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            -29 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_quecto_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_quecto_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            -28 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_quecto_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_quecto_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            -27 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_ronto_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_ronto_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            -26 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_ronto_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_ronto_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            -25 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_ronto_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_ronto_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            -24 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_yocto_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_yocto_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            -23 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_yocto_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_yocto_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            -22 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_yocto_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_yocto_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            -21 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_zepto_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_zepto_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            -20 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_zepto_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_zepto_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            -19 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_zepto_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_zepto_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            -18 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_atto_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_atto_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            -17 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_atto_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_atto_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            -16 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_atto_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_atto_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            -15 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_femto_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_femto_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            -14 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_femto_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_femto_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            -13 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_femto_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_femto_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            -12 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_pico_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_pico_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            -11 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_pico_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_pico_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            -10 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_pico_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_pico_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            -9 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_nano_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_nano_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            -8 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_nano_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_nano_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            -7 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_nano_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_nano_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            -6 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_micro_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_micro_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            -5 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_micro_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_micro_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            -4 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_micro_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_micro_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            -3 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_milli_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_milli_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            -2 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_milli_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_milli_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            -1 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_milli_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_milli_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            0 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            1 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            2 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            3 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_kilo_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_kilo_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            4 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_kilo_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_kilo_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            5 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_kilo_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_kilo_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            6 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_mega_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_mega_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            7 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_mega_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_mega_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            8 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_mega_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_mega_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            9 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_giga_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_giga_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            10 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_giga_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_giga_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            11 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_giga_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_giga_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            12 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_tera_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_tera_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            13 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_tera_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_tera_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            14 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_tera_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_tera_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            15 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_peta_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_peta_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            16 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_peta_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_peta_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            17 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_peta_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_peta_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            18 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_exa_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_exa_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            19 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_exa_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_exa_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            20 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_exa_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_exa_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            21 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_zetta_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_zetta_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            22 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_zetta_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_zetta_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            23 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_zetta_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_zetta_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            24 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_yotta_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_yotta_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            25 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_yotta_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_yotta_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            26 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_yotta_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_yotta_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            27 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_ronna_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_ronna_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            28 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_ronna_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_ronna_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            29 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_ronna_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_ronna_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            30 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_quetta_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_quetta_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            31 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_quetta_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_quetta_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            32 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_quetta_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_quetta_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            33 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_quetta_meter_1000.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_quetta_meter_1000 = Some(loader.chunk_owner_id().clone());
            }
            34 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_quetta_meter_10000.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_quetta_meter_10000 = Some(loader.chunk_owner_id().clone());
            }
            35 => {
                let (loader_transform, loader) = match chunk_loader_query_scale_quetta_meter_100000.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => {
                        unreachable!(
                            "Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.",
                            loader_entity
                        );
                    }
                };
                let loader_position = loader_transform.translation.truncate();
                let loader_radius = loader.radius;

                removed_owner_id_scale_quetta_meter_100000 = Some(loader.chunk_owner_id().clone());
            }
        }
    }

    let handle = composite_workflow!(
        OnRemoveChunkLoader,
        //move in loader_position: Vec2,
        //move in loader_radius: u32,
        move in removed_owner_id_scale_quecto_meter_000001: Option<ChunkOwnerId<ScaleQuectoMeter000001>>,
        move in removed_owner_id_scale_quecto_meter_00001: Option<ChunkOwnerId<ScaleQuectoMeter00001>>,
        move in removed_owner_id_scale_quecto_meter_0001: Option<ChunkOwnerId<ScaleQuectoMeter0001>>,
        move in removed_owner_id_scale_quecto_meter_001: Option<ChunkOwnerId<ScaleQuectoMeter001>>,
        move in removed_owner_id_scale_quecto_meter_01: Option<ChunkOwnerId<ScaleQuectoMeter01>>,
        move in removed_owner_id_scale_quecto_meter_1: Option<ChunkOwnerId<ScaleQuectoMeter1>>,
        move in removed_owner_id_scale_quecto_meter_10: Option<ChunkOwnerId<ScaleQuectoMeter10>>,
        move in removed_owner_id_scale_quecto_meter_100: Option<ChunkOwnerId<ScaleQuectoMeter100>>,
        move in removed_owner_id_scale_ronto_meter_1: Option<ChunkOwnerId<ScaleRontoMeter1>>,
        move in removed_owner_id_scale_ronto_meter_10: Option<ChunkOwnerId<ScaleRontoMeter10>>,
        move in removed_owner_id_scale_ronto_meter_100: Option<ChunkOwnerId<ScaleRontoMeter100>>,
        move in removed_owner_id_scale_yocto_meter_1: Option<ChunkOwnerId<ScaleYoctoMeter1>>,
        move in removed_owner_id_scale_yocto_meter_10: Option<ChunkOwnerId<ScaleYoctoMeter10>>,
        move in removed_owner_id_scale_yocto_meter_100: Option<ChunkOwnerId<ScaleYoctoMeter100>>,
        move in removed_owner_id_scale_zepto_meter_1: Option<ChunkOwnerId<ScaleZeptoMeter1>>,
        move in removed_owner_id_scale_zepto_meter_10: Option<ChunkOwnerId<ScaleZeptoMeter10>>,
        move in removed_owner_id_scale_zepto_meter_100: Option<ChunkOwnerId<ScaleZeptoMeter100>>,
        move in removed_owner_id_scale_atto_meter_1: Option<ChunkOwnerId<ScaleAttoMeter1>>,
        move in removed_owner_id_scale_atto_meter_10: Option<ChunkOwnerId<ScaleAttoMeter10>>,
        move in removed_owner_id_scale_atto_meter_100: Option<ChunkOwnerId<ScaleAttoMeter100>>,
        move in removed_owner_id_scale_femto_meter_1: Option<ChunkOwnerId<ScaleFemtoMeter1>>,
        move in removed_owner_id_scale_femto_meter_10: Option<ChunkOwnerId<ScaleFemtoMeter10>>,
        move in removed_owner_id_scale_femto_meter_100: Option<ChunkOwnerId<ScaleFemtoMeter100>>,
        move in removed_owner_id_scale_pico_meter_1: Option<ChunkOwnerId<ScalePicoMeter1>>,
        move in removed_owner_id_scale_pico_meter_10: Option<ChunkOwnerId<ScalePicoMeter10>>,
        move in removed_owner_id_scale_pico_meter_100: Option<ChunkOwnerId<ScalePicoMeter100>>,
        move in removed_owner_id_scale_nano_meter_1: Option<ChunkOwnerId<ScaleNanoMeter1>>,
        move in removed_owner_id_scale_nano_meter_10: Option<ChunkOwnerId<ScaleNanoMeter10>>,
        move in removed_owner_id_scale_nano_meter_100: Option<ChunkOwnerId<ScaleNanoMeter100>>,
        move in removed_owner_id_scale_micro_meter_1: Option<ChunkOwnerId<ScaleMicroMeter1>>,
        move in removed_owner_id_scale_micro_meter_10: Option<ChunkOwnerId<ScaleMicroMeter10>>,
        move in removed_owner_id_scale_micro_meter_100: Option<ChunkOwnerId<ScaleMicroMeter100>>,
        move in removed_owner_id_scale_milli_meter_1: Option<ChunkOwnerId<ScaleMilliMeter1>>,
        move in removed_owner_id_scale_milli_meter_10: Option<ChunkOwnerId<ScaleMilliMeter10>>,
        move in removed_owner_id_scale_milli_meter_100: Option<ChunkOwnerId<ScaleMilliMeter100>>,
        move in removed_owner_id_scale_meter_1: Option<ChunkOwnerId<ScaleMeter1>>,
        move in removed_owner_id_scale_meter_10: Option<ChunkOwnerId<ScaleMeter10>>,
        move in removed_owner_id_scale_meter_100: Option<ChunkOwnerId<ScaleMeter100>>,
        move in removed_owner_id_scale_kilo_meter_1: Option<ChunkOwnerId<ScaleKiloMeter1>>,
        move in removed_owner_id_scale_kilo_meter_10: Option<ChunkOwnerId<ScaleKiloMeter10>>,
        move in removed_owner_id_scale_kilo_meter_100: Option<ChunkOwnerId<ScaleKiloMeter100>>,
        move in removed_owner_id_scale_mega_meter_1: Option<ChunkOwnerId<ScaleMegaMeter1>>,
        move in removed_owner_id_scale_mega_meter_10: Option<ChunkOwnerId<ScaleMegaMeter10>>,
        move in removed_owner_id_scale_mega_meter_100: Option<ChunkOwnerId<ScaleMegaMeter100>>,
        move in removed_owner_id_scale_giga_meter_1: Option<ChunkOwnerId<ScaleGigaMeter1>>,
        move in removed_owner_id_scale_giga_meter_10: Option<ChunkOwnerId<ScaleGigaMeter10>>,
        move in removed_owner_id_scale_giga_meter_100: Option<ChunkOwnerId<ScaleGigaMeter100>>,
        move in removed_owner_id_scale_tera_meter_1: Option<ChunkOwnerId<ScaleTeraMeter1>>,
        move in removed_owner_id_scale_tera_meter_10: Option<ChunkOwnerId<ScaleTeraMeter10>>,
        move in removed_owner_id_scale_tera_meter_100: Option<ChunkOwnerId<ScaleTeraMeter100>>,
        move in removed_owner_id_scale_peta_meter_1: Option<ChunkOwnerId<ScalePetaMeter1>>,
        move in removed_owner_id_scale_peta_meter_10: Option<ChunkOwnerId<ScalePetaMeter10>>,
        move in removed_owner_id_scale_peta_meter_100: Option<ChunkOwnerId<ScalePetaMeter100>>,
        move in removed_owner_id_scale_exa_meter_1: Option<ChunkOwnerId<ScaleExaMeter1>>,
        move in removed_owner_id_scale_exa_meter_10: Option<ChunkOwnerId<ScaleExaMeter10>>,
        move in removed_owner_id_scale_exa_meter_100: Option<ChunkOwnerId<ScaleExaMeter100>>,
        move in removed_owner_id_scale_zetta_meter_1: Option<ChunkOwnerId<ScaleZettaMeter1>>,
        move in removed_owner_id_scale_zetta_meter_10: Option<ChunkOwnerId<ScaleZettaMeter10>>,
        move in removed_owner_id_scale_zetta_meter_100: Option<ChunkOwnerId<ScaleZettaMeter100>>,
        move in removed_owner_id_scale_yotta_meter_1: Option<ChunkOwnerId<ScaleYottaMeter1>>,
        move in removed_owner_id_scale_yotta_meter_10: Option<ChunkOwnerId<ScaleYottaMeter10>>,
        move in removed_owner_id_scale_yotta_meter_100: Option<ChunkOwnerId<ScaleYottaMeter100>>,
        move in removed_owner_id_scale_ronna_meter_1: Option<ChunkOwnerId<ScaleRonnaMeter1>>,
        move in removed_owner_id_scale_ronna_meter_10: Option<ChunkOwnerId<ScaleRonnaMeter10>>,
        move in removed_owner_id_scale_ronna_meter_100: Option<ChunkOwnerId<ScaleRonnaMeter100>>,
        move in removed_owner_id_scale_quetta_meter_1: Option<ChunkOwnerId<ScaleQuettaMeter1>>,
        move in removed_owner_id_scale_quetta_meter_10: Option<ChunkOwnerId<ScaleQuettaMeter10>>,
        move in removed_owner_id_scale_quetta_meter_100: Option<ChunkOwnerId<ScaleQuettaMeter100>>,
        move in removed_owner_id_scale_quetta_meter_1000: Option<ChunkOwnerId<ScaleQuettaMeter1000>>,
        move in removed_owner_id_scale_quetta_meter_10000: Option<ChunkOwnerId<ScaleQuettaMeter10000>>,
        move in removed_owner_id_scale_quetta_meter_100000: Option<ChunkOwnerId<ScaleQuettaMeter100000>>,
    {
        warn!("Running composite workflow 'OnRemoveChunkLoader'");

        // let output = workflow!(IO, ChunkLoader::OnRemoveChunkLoader, Input {
        //     chunk_owner_id: owner_id.clone(),
        //     chunk_loader_position: loader_position,
        //     chunk_loader_radius: loader_radius,
        // });
        // // TODO: VERY VERY IMPORTANT: THIS IS TERRIBLE FUCKING SHIT!!!!
        // We already use ChunkLoader::UnloadChunks in the chunk_loader systems, and workflows cannot be used concurrently!
        // workflow!(I, ChunkLoader::UnloadChunks, Input {
        //     inputs: output.unload_chunk_inputs
        // });
        workflow!(I, ChunkLoader::OnRemovedChunkLoader, Input {
            inner_scale_quecto_meter_000001: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleQuectoMeter000001> { chunk_owner_id: removed_owner_id_scale_quecto_meter_000001 },
            inner_scale_quecto_meter_00001: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleQuectoMeter00001> { chunk_owner_id: removed_owner_id_scale_quecto_meter_00001 },
            inner_scale_quecto_meter_0001: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleQuectoMeter0001> { chunk_owner_id: removed_owner_id_scale_quecto_meter_0001 },
            inner_scale_quecto_meter_001: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleQuectoMeter001> { chunk_owner_id: removed_owner_id_scale_quecto_meter_001 },
            inner_scale_quecto_meter_01: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleQuectoMeter01> { chunk_owner_id: removed_owner_id_scale_quecto_meter_01 },
            inner_scale_quecto_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleQuectoMeter1> { chunk_owner_id: removed_owner_id_scale_quecto_meter_1 },
            inner_scale_quecto_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleQuectoMeter10> { chunk_owner_id: removed_owner_id_scale_quecto_meter_10 },
            inner_scale_quecto_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleQuectoMeter100> { chunk_owner_id: removed_owner_id_scale_quecto_meter_100 },
            inner_scale_ronto_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleRontoMeter1> { chunk_owner_id: removed_owner_id_scale_ronto_meter_1 },
            inner_scale_ronto_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleRontoMeter10> { chunk_owner_id: removed_owner_id_scale_ronto_meter_10 },
            inner_scale_ronto_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleRontoMeter100> { chunk_owner_id: removed_owner_id_scale_ronto_meter_100 },
            inner_scale_yocto_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleYoctoMeter1> { chunk_owner_id: removed_owner_id_scale_yocto_meter_1 },
            inner_scale_yocto_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleYoctoMeter10> { chunk_owner_id: removed_owner_id_scale_yocto_meter_10 },
            inner_scale_yocto_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleYoctoMeter100> { chunk_owner_id: removed_owner_id_scale_yocto_meter_100 },
            inner_scale_zepto_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleZeptoMeter1> { chunk_owner_id: removed_owner_id_scale_zepto_meter_1 },
            inner_scale_zepto_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleZeptoMeter10> { chunk_owner_id: removed_owner_id_scale_zepto_meter_10 },
            inner_scale_zepto_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleZeptoMeter100> { chunk_owner_id: removed_owner_id_scale_zepto_meter_100 },
            inner_scale_atto_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleAttoMeter1> { chunk_owner_id: removed_owner_id_scale_atto_meter_1 },
            inner_scale_atto_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleAttoMeter10> { chunk_owner_id: removed_owner_id_scale_atto_meter_10 },
            inner_scale_atto_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleAttoMeter100> { chunk_owner_id: removed_owner_id_scale_atto_meter_100 },
            inner_scale_femto_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleFemtoMeter1> { chunk_owner_id: removed_owner_id_scale_femto_meter_1 },
            inner_scale_femto_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleFemtoMeter10> { chunk_owner_id: removed_owner_id_scale_femto_meter_10 },
            inner_scale_femto_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleFemtoMeter100> { chunk_owner_id: removed_owner_id_scale_femto_meter_100 },
            inner_scale_pico_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScalePicoMeter1> { chunk_owner_id: removed_owner_id_scale_pico_meter_1 },
            inner_scale_pico_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScalePicoMeter10> { chunk_owner_id: removed_owner_id_scale_pico_meter_10 },
            inner_scale_pico_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScalePicoMeter100> { chunk_owner_id: removed_owner_id_scale_pico_meter_100 },
            inner_scale_nano_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleNanoMeter1> { chunk_owner_id: removed_owner_id_scale_nano_meter_1 },
            inner_scale_nano_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleNanoMeter10> { chunk_owner_id: removed_owner_id_scale_nano_meter_10 },
            inner_scale_nano_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleNanoMeter100> { chunk_owner_id: removed_owner_id_scale_nano_meter_100 },
            inner_scale_micro_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleMicroMeter1> { chunk_owner_id: removed_owner_id_scale_micro_meter_1 },
            inner_scale_micro_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleMicroMeter10> { chunk_owner_id: removed_owner_id_scale_micro_meter_10 },
            inner_scale_micro_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleMicroMeter100> { chunk_owner_id: removed_owner_id_scale_micro_meter_100 },
            inner_scale_milli_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleMilliMeter1> { chunk_owner_id: removed_owner_id_scale_milli_meter_1 },
            inner_scale_milli_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleMilliMeter10> { chunk_owner_id: removed_owner_id_scale_milli_meter_10 },
            inner_scale_milli_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleMilliMeter100> { chunk_owner_id: removed_owner_id_scale_milli_meter_100 },
            inner_scale_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleMeter1> { chunk_owner_id: removed_owner_id_scale_meter_1 },
            inner_scale_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleMeter10> { chunk_owner_id: removed_owner_id_scale_meter_10 },
            inner_scale_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleMeter100> { chunk_owner_id: removed_owner_id_scale_meter_100 },
            inner_scale_kilo_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleKiloMeter1> { chunk_owner_id: removed_owner_id_scale_kilo_meter_1 },
            inner_scale_kilo_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleKiloMeter10> { chunk_owner_id: removed_owner_id_scale_kilo_meter_10 },
            inner_scale_kilo_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleKiloMeter100> { chunk_owner_id: removed_owner_id_scale_kilo_meter_100 },
            inner_scale_mega_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleMegaMeter1> { chunk_owner_id: removed_owner_id_scale_mega_meter_1 },
            inner_scale_mega_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleMegaMeter10> { chunk_owner_id: removed_owner_id_scale_mega_meter_10 },
            inner_scale_mega_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleMegaMeter100> { chunk_owner_id: removed_owner_id_scale_mega_meter_100 },
            inner_scale_giga_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleGigaMeter1> { chunk_owner_id: removed_owner_id_scale_giga_meter_1 },
            inner_scale_giga_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleGigaMeter10> { chunk_owner_id: removed_owner_id_scale_giga_meter_10 },
            inner_scale_giga_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleGigaMeter100> { chunk_owner_id: removed_owner_id_scale_giga_meter_100 },
            inner_scale_tera_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleTeraMeter1> { chunk_owner_id: removed_owner_id_scale_tera_meter_1 },
            inner_scale_tera_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleTeraMeter10> { chunk_owner_id: removed_owner_id_scale_tera_meter_10 },
            inner_scale_tera_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleTeraMeter100> { chunk_owner_id: removed_owner_id_scale_tera_meter_100 },
            inner_scale_peta_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScalePetaMeter1> { chunk_owner_id: removed_owner_id_scale_peta_meter_1 },
            inner_scale_peta_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScalePetaMeter10> { chunk_owner_id: removed_owner_id_scale_peta_meter_10 },
            inner_scale_peta_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScalePetaMeter100> { chunk_owner_id: removed_owner_id_scale_peta_meter_100 },
            inner_scale_exa_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleExaMeter1> { chunk_owner_id: removed_owner_id_scale_exa_meter_1 },
            inner_scale_exa_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleExaMeter10> { chunk_owner_id: removed_owner_id_scale_exa_meter_10 },
            inner_scale_exa_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleExaMeter100> { chunk_owner_id: removed_owner_id_scale_exa_meter_100 },
            inner_scale_zetta_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleZettaMeter1> { chunk_owner_id: removed_owner_id_scale_zetta_meter_1 },
            inner_scale_zetta_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleZettaMeter10> { chunk_owner_id: removed_owner_id_scale_zetta_meter_10 },
            inner_scale_zetta_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleZettaMeter100> { chunk_owner_id: removed_owner_id_scale_zetta_meter_100 },
            inner_scale_yotta_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleYottaMeter1> { chunk_owner_id: removed_owner_id_scale_yotta_meter_1 },
            inner_scale_yotta_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleYottaMeter10> { chunk_owner_id: removed_owner_id_scale_yotta_meter_10 },
            inner_scale_yotta_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleYottaMeter100> { chunk_owner_id: removed_owner_id_scale_yotta_meter_100 },
            inner_scale_ronna_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleRonnaMeter1> { chunk_owner_id: removed_owner_id_scale_ronna_meter_1 },
            inner_scale_ronna_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleRonnaMeter10> { chunk_owner_id: removed_owner_id_scale_ronna_meter_10 },
            inner_scale_ronna_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleRonnaMeter100> { chunk_owner_id: removed_owner_id_scale_ronna_meter_100 },
            inner_scale_quetta_meter_1: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleQuettaMeter1> { chunk_owner_id: removed_owner_id_scale_quetta_meter_1 },
            inner_scale_quetta_meter_10: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleQuettaMeter10> { chunk_owner_id: removed_owner_id_scale_quetta_meter_10 },
            inner_scale_quetta_meter_100: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleQuettaMeter100> { chunk_owner_id: removed_owner_id_scale_quetta_meter_100 },
            inner_scale_quetta_meter_1000: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleQuettaMeter1000> { chunk_owner_id: removed_owner_id_scale_quetta_meter_1000 },
            inner_scale_quetta_meter_10000: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleQuettaMeter10000> { chunk_owner_id: removed_owner_id_scale_quetta_meter_10000 },
            inner_scale_quetta_meter_100000: crate::chunk_loader::workflows::external::on_removed_chunk_loader::Input::<ScaleQuettaMeter100000> { chunk_owner_id: removed_owner_id_scale_quetta_meter_100000 },
        });
    });

    handle_composite_workflow_return_later(handle, |_ctx| {
        composite_workflow_return!();
        warn!("Finished composite workflow 'OnRemoveChunkLoader'");
    });
}
