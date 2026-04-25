use crate::bevy::prelude::World as BevyWorld;

pub type ResourceInsertDispatchFn = fn(&mut BevyWorld, String);
pub type ResourceInitDispatchFn = fn(&mut BevyWorld);
pub type ResourceGetDispatchFn = fn(&mut BevyWorld) -> Option<String>;
pub type ResourceGetMutDispatchFn = fn(&mut BevyWorld) -> Option<String>;
pub type ResourceRemoveDispatchFn = fn(&mut BevyWorld) -> Option<String>;

inventory::collect!(ResourceInsertDispatchEntry);
pub struct ResourceInsertDispatchEntry {
    pub signature_id: &'static str,
    pub resource_type_id: &'static str,
    pub dispatch: ResourceInsertDispatchFn,
}

inventory::collect!(ResourceInitDispatchEntry);
pub struct ResourceInitDispatchEntry {
    pub signature_id: &'static str,
    pub resource_type_id: &'static str,
    pub dispatch: ResourceInitDispatchFn,
}

inventory::collect!(ResourceGetDispatchEntry);
pub struct ResourceGetDispatchEntry {
    pub signature_id: &'static str,
    pub resource_type_id: &'static str,
    pub dispatch: ResourceGetDispatchFn,
}

inventory::collect!(ResourceGetMutDispatchEntry);
pub struct ResourceGetMutDispatchEntry {
    pub signature_id: &'static str,
    pub resource_type_id: &'static str,
    pub dispatch: ResourceGetMutDispatchFn,
}

inventory::collect!(ResourceRemoveDispatchEntry);
pub struct ResourceRemoveDispatchEntry {
    pub signature_id: &'static str,
    pub resource_type_id: &'static str,
    pub dispatch: ResourceRemoveDispatchFn,
}
