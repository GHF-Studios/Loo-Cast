use bevy::prelude::*;

pub fn poll_routines(world: &mut World) {
    let mut routines = world.resource_mut::<RoutineManager>();

    for routine in routines.iter_mut() {
        let mut pinned_future = Pin::new(&mut routine.future);

        match pinned_future.as_mut().poll(&mut Context::from_waker(noop_waker_ref())) {
            Poll::Ready(()) => routine.complete = true,
            Poll::Pending => {}
        }
    }

    routines.retain(|r| !r.complete );
}
