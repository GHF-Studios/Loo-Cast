use std::cell::RefCell;
use anymap::AnyMap;

thread_local! {
    static WORKFLOW_CONTEXT: RefCell<AnyMap> = RefCell::new(AnyMap::new());
}

pub fn set_context<T: 'static>(val: T) {
    WORKFLOW_CONTEXT.with(|ctx| ctx.borrow_mut().insert::<Option<T>>(Some(val)));
}

pub fn get_context<T: 'static>() -> T {
    let mut context: Option<T> = None;
    WORKFLOW_CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        let ctx = ctx.get_mut::<Option<T>>().unwrap_or_else(|| unreachable!("Context not found"));
        let ctx = ctx.take().unwrap_or_else(|| unreachable!("Context not found"));
        context = Some(ctx);
    });
    context.unwrap_or_else(|| unreachable!("Context not found"))
}

pub fn clear_all_context() {
    WORKFLOW_CONTEXT.with(|ctx| {
        ctx.borrow_mut().clear();
    });
}
