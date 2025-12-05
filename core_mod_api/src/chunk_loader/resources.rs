use bevy::prelude::*;
use std::collections::HashSet;

use super::types::*;

#[derive(Resource, Reflect, Clone, Debug, Default)]
#[reflect(Resource)]
pub struct RemovedChunkLoaders(pub HashSet<RemovedChunkLoader>);

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource)]
pub struct RemovedChunkLoaderObservationQueue(pub HashSet<RemovedChunkLoaderObservation>);
