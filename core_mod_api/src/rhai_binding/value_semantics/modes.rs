#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TypeValueSemantics {
    #[default]
    CloneOnMove,
    ScopedMut,
    PersistentRef,
    PersistentMut,
}

pub trait GetTypeValueSemantics {
    const VALUE_SEMANTICS: TypeValueSemantics = TypeValueSemantics::CloneOnMove;
}
