use crate::bevy::prelude::Entity as BevyEntity;

pub trait EntityRefApi {
    fn id(&self) -> BevyEntity;
}

pub trait EntityMutApi {
    fn id(&self) -> BevyEntity;
}

pub trait EntityWorldMutApi {
    fn id(&self) -> BevyEntity;
}
