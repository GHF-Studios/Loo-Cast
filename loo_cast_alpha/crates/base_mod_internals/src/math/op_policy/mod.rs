/// Runtime operation-policy wrapper for mathematically intrinsic mode variants.
///
/// `DeferToGlobal` is the default route for runtime-configured policy.
/// `Explicit` allows call-site override when the operation surface exposes it.
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub enum OpPolicy<T = ()> {
    #[default]
    DeferToGlobal,
    Explicit(T),
}
