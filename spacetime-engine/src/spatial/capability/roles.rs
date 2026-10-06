//! Scale-local capability role vocabulary.

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UsfScaleRoleMask(u16);

impl UsfScaleRoleMask {
    pub const NONE: Self = Self(0);
    pub const REALIZATION: Self = Self(1 << 0);
    pub const PRESENTATION: Self = Self(1 << 1);
    /// Local backend contact/response capability.
    ///
    /// This role may participate in solver impulses/manifolds.
    pub const COLLISION: Self = Self(1 << 2);
    pub const EDITING: Self = Self(1 << 3);
    /// Conservative swept-query/refinement support.
    ///
    /// `COLLISION_QUERY` is explicitly non-authoritative: satisfying this role
    /// may produce candidate TOI intervals/results, but must never apply an
    /// impulse or mutate canonical motion.
    pub const COLLISION_QUERY: Self = Self(1 << 4);

    pub const fn bits(self) -> u16 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, role: Self) -> bool {
        (self.0 & role.0) == role.0
    }
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
