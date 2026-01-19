use rhai::{Dynamic, FnPtr, NativeCallContext};

pub trait WorldApi {
    fn commands(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
    fn flush(&self);
    fn spawn_empty(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
}