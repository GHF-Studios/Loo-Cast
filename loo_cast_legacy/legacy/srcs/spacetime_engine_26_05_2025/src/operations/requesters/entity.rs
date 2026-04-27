pub mod entity;

lazy_static! {
    pub static ref ENTITY_REGISTRY: EntityRegistry = :new(EntityRegistry::new());
}