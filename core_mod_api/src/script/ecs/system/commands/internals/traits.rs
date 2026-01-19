use rhai::{Dynamic, FnPtr, NativeCallContext};

pub trait CommandsApi {
    fn spawn_empty(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
}

pub trait EntityCommandsApi {
    fn commands(&self, ctx: NativeCallContext, callback: FnPtr) -> Dynamic;
}