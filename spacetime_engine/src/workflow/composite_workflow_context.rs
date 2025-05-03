use std::cell::RefCell;
use anymap::AnyMap;

thread_local! {
    static WORKFLOW_CONTEXT: RefCell<AnyMap> = RefCell::new(AnyMap::new());
}

pub fn set_context<T: 'static>(val: T) {
    WORKFLOW_CONTEXT.with(|ctx| ctx.borrow_mut().insert::<T>(val));
}

pub fn get_context<T: 'static + Clone>() -> T
{
    WORKFLOW_CONTEXT.with(|ctx| ctx.borrow().get::<T>().expect("Missing context").clone())
}

pub fn clear_all_context() {
    WORKFLOW_CONTEXT.with(|ctx| {
        ctx.borrow_mut().clear();
    });
}

#[macro_export]
macro_rules! define_composite_workflow {
    ($($captured:expr),+ , $name:ident $block:block) => {{
        $(
            $crate::set_context($captured);
        )+
        define_composite_workflow_inner!($name:ident $block)
        $crate::clear_context::<_>();
    }};

    ($name:ident $block:block) => {
        define_composite_workflow_inner!($name:ident $block)
    };
}
