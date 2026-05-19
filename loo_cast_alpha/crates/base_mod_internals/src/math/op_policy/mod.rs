/// Runtime operation-policy wrapper for mathematically intrinsic mode variants.
///
/// `DeferToGlobal` is the default route for runtime-configured policy.
/// `Explicit` allows call-site override when the operation surface exposes it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OpPolicy<T = ()> {
    DeferToGlobal,
    Explicit(T),
}

impl<T> Default for OpPolicy<T> {
    fn default() -> Self {
        Self::DeferToGlobal
    }
}
