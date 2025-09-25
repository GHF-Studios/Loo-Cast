use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
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

#[derive(SystemParam)]
struct ProcessingSystemChunkLoaderQueries<'w, 's> {
    pub chunk_loader_query_scale_quecto_meter_000001: Query<'w, 's, &'static ChunkLoader<ScaleQuectoMeter000001>>,
    pub chunk_loader_query_scale_quecto_meter_00001: Query<'w, 's, &'static ChunkLoader<ScaleQuectoMeter00001>>,
    pub chunk_loader_query_scale_quecto_meter_0001: Query<'w, 's, &'static ChunkLoader<ScaleQuectoMeter0001>>,
    pub chunk_loader_query_scale_quecto_meter_001: Query<'w, 's, &'static ChunkLoader<ScaleQuectoMeter001>>,
    pub chunk_loader_query_scale_quecto_meter_01: Query<'w, 's, &'static ChunkLoader<ScaleQuectoMeter01>>,
    pub chunk_loader_query_scale_quecto_meter_1: Query<'w, 's, &'static ChunkLoader<ScaleQuectoMeter1>>,
    pub chunk_loader_query_scale_quecto_meter_10: Query<'w, 's, &'static ChunkLoader<ScaleQuectoMeter10>>,
    pub chunk_loader_query_scale_quecto_meter_100: Query<'w, 's, &'static ChunkLoader<ScaleQuectoMeter100>>,
    pub chunk_loader_query_scale_ronto_meter_1: Query<'w, 's, &'static ChunkLoader<ScaleRontoMeter1>>,
    pub chunk_loader_query_scale_ronto_meter_10: Query<'w, 's, &'static ChunkLoader<ScaleRontoMeter10>>,
    pub chunk_loader_query_scale_ronto_meter_100: Query<'w, 's, &'static ChunkLoader<ScaleRontoMeter100>>,
    pub chunk_loader_query_scale_yocto_meter_1: Query<'w, 's, &'static ChunkLoader<ScaleYoctoMeter1>>,
    pub chunk_loader_query_scale_yocto_meter_10: Query<'w, 's, &'static ChunkLoader<ScaleYoctoMeter10>>,
    pub chunk_loader_query_scale_yocto_meter_100: Query<'w, 's, &'static ChunkLoader<ScaleYoctoMeter100>>,
    pub chunk_loader_query_scale_zepto_meter_1: Query<'w, 's, &'static ChunkLoader<ScaleZeptoMeter1>>,
    pub chunk_loader_query_scale_zepto_meter_10: Query<'w, 's, &'static ChunkLoader<ScaleZeptoMeter10>>,
    pub chunk_loader_query_scale_zepto_meter_100: Query<'w, 's, &'static ChunkLoader<ScaleZeptoMeter100>>,
    pub chunk_loader_query_scale_atto_meter_1: Query<'w, 's, &'static ChunkLoader<ScaleAttoMeter1>>,
    pub chunk_loader_query_scale_atto_meter_10: Query<'w, 's, &'static ChunkLoader<ScaleAttoMeter10>>,
    pub chunk_loader_query_scale_atto_meter_100: Query<'w, 's, &'static ChunkLoader<ScaleAttoMeter100>>,
    pub chunk_loader_query_scale_femto_meter_1: Query<'w, 's, &'static ChunkLoader<ScaleFemtoMeter1>>,
    pub chunk_loader_query_scale_femto_meter_10: Query<'w, 's, &'static ChunkLoader<ScaleFemtoMeter10>>,
    pub chunk_loader_query_scale_femto_meter_100: Query<'w, 's, &'static ChunkLoader<ScaleFemtoMeter100>>,
    pub chunk_loader_query_scale_pico_meter_1: Query<'w, 's, &'static ChunkLoader<ScalePicoMeter1>>,
    pub chunk_loader_query_scale_pico_meter_10: Query<'w, 's, &'static ChunkLoader<ScalePicoMeter10>>,
    pub chunk_loader_query_scale_pico_meter_100: Query<'w, 's, &'static ChunkLoader<ScalePicoMeter100>>,
    pub chunk_loader_query_scale_nano_meter_1: Query<'w, 's, &'static ChunkLoader<ScaleNanoMeter1>>,
    pub chunk_loader_query_scale_nano_meter_10: Query<'w, 's, &'static ChunkLoader<ScaleNanoMeter10>>,
    pub chunk_loader_query_scale_nano_meter_100: Query<'w, 's, &'static ChunkLoader<ScaleNanoMeter100>>,
    pub chunk_loader_query_scale_micro_meter_1: Query<'w, 's, &'static ChunkLoader<ScaleMicroMeter1>>,
    pub chunk_loader_query_scale_micro_meter_10: Query<'w, 's, &'static ChunkLoader<ScaleMicroMeter10>>,
    pub chunk_loader_query_scale_micro_meter_100: Query<'w, 's, &'static ChunkLoader<ScaleMicroMeter100>>,
    pub chunk_loader_query_scale_milli_meter_1: Query<'w, 's, &'static ChunkLoader<ScaleMilliMeter1>>,
    pub chunk_loader_query_scale_milli_meter_10: Query<'w, 's, &'static ChunkLoader<ScaleMilliMeter10>>,
    pub chunk_loader_query_scale_milli_meter_100: Query<'w, 's, &'static ChunkLoader<ScaleMilliMeter100>>,
    pub chunk_loader_query_scale_meter_1: Query<'w, 's, &'static ChunkLoader<ScaleMeter1>>,
    pub chunk_loader_query_scale_meter_10: Query<'w, 's, &'static ChunkLoader<ScaleMeter10>>,
    pub chunk_loader_query_scale_meter_100: Query<'w, 's, &'static ChunkLoader<ScaleMeter100>>,
    pub chunk_loader_query_scale_kilo_meter_1: Query<'w, 's, &'static ChunkLoader<ScaleKiloMeter1>>,
    pub chunk_loader_query_scale_kilo_meter_10: Query<'w, 's, &'static ChunkLoader<ScaleKiloMeter10>>,
    pub chunk_loader_query_scale_kilo_meter_100: Query<'w, 's, &'static ChunkLoader<ScaleKiloMeter100>>,
    pub chunk_loader_query_scale_mega_meter_1: Query<'w, 's, &'static ChunkLoader<ScaleMegaMeter1>>,
    pub chunk_loader_query_scale_mega_meter_10: Query<'w, 's, &'static ChunkLoader<ScaleMegaMeter10>>,
    pub chunk_loader_query_scale_mega_meter_100: Query<'w, 's, &'static ChunkLoader<ScaleMegaMeter100>>,
    pub chunk_loader_query_scale_giga_meter_1: Query<'w, 's, &'static ChunkLoader<ScaleGigaMeter1>>,
    pub chunk_loader_query_scale_giga_meter_10: Query<'w, 's, &'static ChunkLoader<ScaleGigaMeter10>>,
    pub chunk_loader_query_scale_giga_meter_100: Query<'w, 's, &'static ChunkLoader<ScaleGigaMeter100>>,
    pub chunk_loader_query_scale_tera_meter_1: Query<'w, 's, &'static ChunkLoader<ScaleTeraMeter1>>,
    pub chunk_loader_query_scale_tera_meter_10: Query<'w, 's, &'static ChunkLoader<ScaleTeraMeter10>>,
    pub chunk_loader_query_scale_tera_meter_100: Query<'w, 's, &'static ChunkLoader<ScaleTeraMeter100>>,
    pub chunk_loader_query_scale_peta_meter_1: Query<'w, 's, &'static ChunkLoader<ScalePetaMeter1>>,
    pub chunk_loader_query_scale_peta_meter_10: Query<'w, 's, &'static ChunkLoader<ScalePetaMeter10>>,
    pub chunk_loader_query_scale_peta_meter_100: Query<'w, 's, &'static ChunkLoader<ScalePetaMeter100>>,
    pub chunk_loader_query_scale_exa_meter_1: Query<'w, 's, &'static ChunkLoader<ScaleExaMeter1>>,
    pub chunk_loader_query_scale_exa_meter_10: Query<'w, 's, &'static ChunkLoader<ScaleExaMeter10>>,
    pub chunk_loader_query_scale_exa_meter_100: Query<'w, 's, &'static ChunkLoader<ScaleExaMeter100>>,
    pub chunk_loader_query_scale_zetta_meter_1: Query<'w, 's, &'static ChunkLoader<ScaleZettaMeter1>>,
    pub chunk_loader_query_scale_zetta_meter_10: Query<'w, 's, &'static ChunkLoader<ScaleZettaMeter10>>,
    pub chunk_loader_query_scale_zetta_meter_100: Query<'w, 's, &'static ChunkLoader<ScaleZettaMeter100>>,
    pub chunk_loader_query_scale_yotta_meter_1: Query<'w, 's, &'static ChunkLoader<ScaleYottaMeter1>>,
    pub chunk_loader_query_scale_yotta_meter_10: Query<'w, 's, &'static ChunkLoader<ScaleYottaMeter10>>,
    pub chunk_loader_query_scale_yotta_meter_100: Query<'w, 's, &'static ChunkLoader<ScaleYottaMeter100>>,
    pub chunk_loader_query_scale_ronna_meter_1: Query<'w, 's, &'static ChunkLoader<ScaleRonnaMeter1>>,
    pub chunk_loader_query_scale_ronna_meter_10: Query<'w, 's, &'static ChunkLoader<ScaleRonnaMeter10>>,
    pub chunk_loader_query_scale_ronna_meter_100: Query<'w, 's, &'static ChunkLoader<ScaleRonnaMeter100>>,
    pub chunk_loader_query_scale_quetta_meter_1: Query<'w, 's, &'static ChunkLoader<ScaleQuettaMeter1>>,
    pub chunk_loader_query_scale_quetta_meter_10: Query<'w, 's, &'static ChunkLoader<ScaleQuettaMeter10>>,
    pub chunk_loader_query_scale_quetta_meter_100: Query<'w, 's, &'static ChunkLoader<ScaleQuettaMeter100>>,
    pub chunk_loader_query_scale_quetta_meter_1000: Query<'w, 's, &'static ChunkLoader<ScaleQuettaMeter1000>>,
    pub chunk_loader_query_scale_quetta_meter_10000: Query<'w, 's, &'static ChunkLoader<ScaleQuettaMeter10000>>,
    pub chunk_loader_query_scale_quetta_meter_100000: Query<'w, 's, &'static ChunkLoader<ScaleQuettaMeter100000>>,
}

// TODO: MAJOR: This silently drops observed chunk loader removals in the same scale if one is already in-progress composite-workflow-wise, so for now:
// Concurrent chunk loader removals of the same scale are unsound!
#[tracing::instrument(skip_all)]
pub(crate) fn on_remove_chunk_loader_observation_queue_processing_system(
    chunk_loader_queries: ProcessingSystemChunkLoaderQueries,
    mut queue: ResMut<RemovedChunkLoaderObservationQueue>,
) {
    let mut removed_owner_id_scale_quecto_meter_000001 = None;
    let mut removed_owner_id_scale_quecto_meter_00001 = None;
    let mut removed_owner_id_scale_quecto_meter_0001 = None;
    let mut removed_owner_id_scale_quecto_meter_001 = None;
    let mut removed_owner_id_scale_quecto_meter_01 = None;
    let mut removed_owner_id_scale_quecto_meter_1 = None;
    let mut removed_owner_id_scale_quecto_meter_10 = None;
    let mut removed_owner_id_scale_quecto_meter_100 = None;
    let mut removed_owner_id_scale_ronto_meter_1 = None;
    let mut removed_owner_id_scale_ronto_meter_10 = None;
    let mut removed_owner_id_scale_ronto_meter_100 = None;
    let mut removed_owner_id_scale_yocto_meter_1 = None;
    let mut removed_owner_id_scale_yocto_meter_10 = None;
    let mut removed_owner_id_scale_yocto_meter_100 = None;
    let mut removed_owner_id_scale_zepto_meter_1 = None;
    let mut removed_owner_id_scale_zepto_meter_10 = None;
    let mut removed_owner_id_scale_zepto_meter_100 = None;
    let mut removed_owner_id_scale_atto_meter_1 = None;
    let mut removed_owner_id_scale_atto_meter_10 = None;
    let mut removed_owner_id_scale_atto_meter_100 = None;
    let mut removed_owner_id_scale_femto_meter_1 = None;
    let mut removed_owner_id_scale_femto_meter_10 = None;
    let mut removed_owner_id_scale_femto_meter_100 = None;
    let mut removed_owner_id_scale_pico_meter_1 = None;
    let mut removed_owner_id_scale_pico_meter_10 = None;
    let mut removed_owner_id_scale_pico_meter_100 = None;
    let mut removed_owner_id_scale_nano_meter_1 = None;
    let mut removed_owner_id_scale_nano_meter_10 = None;
    let mut removed_owner_id_scale_nano_meter_100 = None;
    let mut removed_owner_id_scale_micro_meter_1 = None;
    let mut removed_owner_id_scale_micro_meter_10 = None;
    let mut removed_owner_id_scale_micro_meter_100 = None;
    let mut removed_owner_id_scale_milli_meter_1 = None;
    let mut removed_owner_id_scale_milli_meter_10 = None;
    let mut removed_owner_id_scale_milli_meter_100 = None;
    let mut removed_owner_id_scale_meter_1 = None;
    let mut removed_owner_id_scale_meter_10 = None;
    let mut removed_owner_id_scale_meter_100 = None;
    let mut removed_owner_id_scale_kilo_meter_1 = None;
    let mut removed_owner_id_scale_kilo_meter_10 = None;
    let mut removed_owner_id_scale_kilo_meter_100 = None;
    let mut removed_owner_id_scale_mega_meter_1 = None;
    let mut removed_owner_id_scale_mega_meter_10 = None;
    let mut removed_owner_id_scale_mega_meter_100 = None;
    let mut removed_owner_id_scale_giga_meter_1 = None;
    let mut removed_owner_id_scale_giga_meter_10 = None;
    let mut removed_owner_id_scale_giga_meter_100 = None;
    let mut removed_owner_id_scale_tera_meter_1 = None;
    let mut removed_owner_id_scale_tera_meter_10 = None;
    let mut removed_owner_id_scale_tera_meter_100 = None;
    let mut removed_owner_id_scale_peta_meter_1 = None;
    let mut removed_owner_id_scale_peta_meter_10 = None;
    let mut removed_owner_id_scale_peta_meter_100 = None;
    let mut removed_owner_id_scale_exa_meter_1 = None;
    let mut removed_owner_id_scale_exa_meter_10 = None;
    let mut removed_owner_id_scale_exa_meter_100 = None;
    let mut removed_owner_id_scale_zetta_meter_1 = None;
    let mut removed_owner_id_scale_zetta_meter_10 = None;
    let mut removed_owner_id_scale_zetta_meter_100 = None;
    let mut removed_owner_id_scale_yotta_meter_1 = None;
    let mut removed_owner_id_scale_yotta_meter_10 = None;
    let mut removed_owner_id_scale_yotta_meter_100 = None;
    let mut removed_owner_id_scale_ronna_meter_1 = None;
    let mut removed_owner_id_scale_ronna_meter_10 = None;
    let mut removed_owner_id_scale_ronna_meter_100 = None;
    let mut removed_owner_id_scale_quetta_meter_1 = None;
    let mut removed_owner_id_scale_quetta_meter_10 = None;
    let mut removed_owner_id_scale_quetta_meter_100 = None;
    let mut removed_owner_id_scale_quetta_meter_1000 = None;
    let mut removed_owner_id_scale_quetta_meter_10000 = None;
    let mut removed_owner_id_scale_quetta_meter_100000 = None;

    for RemovedChunkLoaderObservation { entity: loader_entity, scale } in std::mem::take(&mut queue.0).into_iter() {
        match scale {
            i8::MIN..=-36_i8 | 36_i8..=i8::MAX => {
                unreachable!("Unsupported scale 'Scale {{ scale_factor_exponent: {} }}' for chunk loader removal observation processing.", scale);
            }
            -35_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_quecto_meter_000001.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_quecto_meter_000001 = Some(loader.chunk_owner_id().clone());
            }
            -34_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_quecto_meter_00001.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_quecto_meter_00001 = Some(loader.chunk_owner_id().clone());
            }
            -33_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_quecto_meter_0001.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_quecto_meter_0001 = Some(loader.chunk_owner_id().clone());
            }
            -32_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_quecto_meter_001.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_quecto_meter_001 = Some(loader.chunk_owner_id().clone());
            }
            -31_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_quecto_meter_01.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_quecto_meter_01 = Some(loader.chunk_owner_id().clone());
            }
            -30_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_quecto_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_quecto_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            -29_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_quecto_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_quecto_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            -28_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_quecto_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_quecto_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            -27_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_ronto_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_ronto_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            -26_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_ronto_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_ronto_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            -25_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_ronto_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_ronto_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            -24_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_yocto_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_yocto_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            -23_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_yocto_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_yocto_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            -22_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_yocto_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_yocto_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            -21_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_zepto_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_zepto_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            -20_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_zepto_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_zepto_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            -19_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_zepto_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_zepto_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            -18_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_atto_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_atto_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            -17_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_atto_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_atto_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            -16_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_atto_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_atto_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            -15_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_femto_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_femto_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            -14_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_femto_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_femto_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            -13_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_femto_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_femto_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            -12_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_pico_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_pico_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            -11_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_pico_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_pico_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            -10_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_pico_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_pico_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            -9_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_nano_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_nano_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            -8_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_nano_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_nano_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            -7_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_nano_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_nano_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            -6_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_micro_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_micro_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            -5_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_micro_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_micro_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            -4_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_micro_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_micro_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            -3_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_milli_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_milli_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            -2_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_milli_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_milli_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            -1_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_milli_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_milli_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            0_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            1_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            2_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            3_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_kilo_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_kilo_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            4_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_kilo_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_kilo_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            5_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_kilo_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_kilo_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            6_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_mega_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_mega_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            7_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_mega_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_mega_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            8_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_mega_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_mega_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            9_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_giga_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_giga_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            10_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_giga_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_giga_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            11_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_giga_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_giga_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            12_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_tera_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_tera_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            13_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_tera_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_tera_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            14_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_tera_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_tera_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            15_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_peta_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_peta_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            16_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_peta_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_peta_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            17_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_peta_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_peta_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            18_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_exa_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_exa_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            19_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_exa_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_exa_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            20_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_exa_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_exa_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            21_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_zetta_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_zetta_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            22_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_zetta_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_zetta_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            23_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_zetta_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_zetta_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            24_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_yotta_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_yotta_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            25_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_yotta_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_yotta_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            26_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_yotta_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_yotta_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            27_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_ronna_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_ronna_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            28_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_ronna_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_ronna_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            29_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_ronna_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_ronna_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            30_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_quetta_meter_1.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_quetta_meter_1 = Some(loader.chunk_owner_id().clone());
            }
            31_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_quetta_meter_10.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_quetta_meter_10 = Some(loader.chunk_owner_id().clone());
            }
            32_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_quetta_meter_100.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_quetta_meter_100 = Some(loader.chunk_owner_id().clone());
            }
            33_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_quetta_meter_1000.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_quetta_meter_1000 = Some(loader.chunk_owner_id().clone());
            }
            34_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_quetta_meter_10000.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

                removed_owner_id_scale_quetta_meter_10000 = Some(loader.chunk_owner_id().clone());
            }
            35_i8 => {
                let loader = match chunk_loader_queries.chunk_loader_query_scale_quetta_meter_100000.get(loader_entity) {
                    Ok(value) => value,
                    Err(_) => unreachable!("Failed to remove chunk loader {:?}: Chunk Loader Query did not include it at the present time.", loader_entity)
                };

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
