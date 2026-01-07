use rhai::Dynamic;

pub trait FromDynamic: Sized + Send + Sync + 'static {
    fn from_dynamic(params: Dynamic) -> Self;
}
