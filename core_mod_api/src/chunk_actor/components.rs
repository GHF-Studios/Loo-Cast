use bevy::prelude::*;
use std::marker::PhantomData;

use crate::usf::scale::Scale;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ChunkActor<S: Scale>(#[reflect(ignore)] pub PhantomData<S>);
impl<S: Scale> Default for ChunkActor<S> {
    fn default() -> Self {
        ChunkActor(PhantomData::<S>)
    }
}
