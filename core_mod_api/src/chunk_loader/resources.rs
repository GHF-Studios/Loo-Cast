use bevy::prelude::*;
use std::collections::HashSet;

use crate::usf::scale::Scale;

use super::types::*;

#[derive(Resource, Reflect, Clone, Debug, Default)]
#[reflect(Resource)]
pub struct RemovedChunkLoaders<S: Scale>(pub HashSet<RemovedChunkLoader<S>>);

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource)]
pub(super) struct RemovedChunkLoaderObservationQueue(pub HashSet<RemovedChunkLoaderObservation>);