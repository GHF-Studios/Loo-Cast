use bevy::prelude::*;
use std::marker::PhantomData;

use crate::usf::scale::ConstScale;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ChunkActor<S: ConstScale>(#[reflect(ignore)] pub PhantomData<S>);
impl<S: ConstScale> Default for ChunkActor<S> {
    fn default() -> Self {
        ChunkActor(PhantomData::<S>)
    }
}
