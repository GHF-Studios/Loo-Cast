#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TypeValueSemantics {
    #[default]
    Clone,
    Owned,
    Ref,
    Mut,
    ScopedOwned,
    ScopedRef,
    ScopedMut,
}

pub trait GetTypeValueSemantics {
    const VALUE_SEMANTICS: TypeValueSemantics = TypeValueSemantics::Clone;
}

pub mod consts {
    pub const CLONE: u8 = 0;
    pub const OWNED: u8 = 1;
    pub const REF: u8 = 2;
    pub const MUT: u8 = 3;
    pub const SCOPED_OWNED: u8 = 4;
    pub const SCOPED_REF: u8 = 5;
    pub const SCOPED_MUT: u8 = 6;
}

pub trait HasTypeValueSemanticsConst<const SEMANTICS: u8> {}
