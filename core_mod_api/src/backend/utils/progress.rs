// use crate::bevy::prelude::*;

// pub(crate) struct ProgressPlugin<U, F>;
// impl<U, F> Plugin for ProgressPlugin<U, F> {
//     fn build(&self, app: &mut App) {
//         app
//             .register_type::<Progress<U, F>>();
//     }
// }

// #[derive(Reflect)]
// TODO: MAJOR: Impl Reflect manually, and add Reflect and other common sensible trait impls/derives to all external workflow types.
pub enum Progress<U, F> {
    Unfinished(U),
    Finished(F),
}
impl<U, F> Progress<U, F> {
    pub fn is_finished(&self) -> bool {
        matches!(self, Progress::Finished(_))
    }

    pub fn is_unfinished(&self) -> bool {
        matches!(self, Progress::Unfinished(_))
    }

    pub fn unwrap_unfinished(self) -> U {
        match self {
            Progress::Unfinished(u) => u,
            Progress::Finished(_) => panic!("called `Progress::unwrap_unfinished()` on a `Finished` value"),
        }
    }

    pub fn unwrap_finished(self) -> F {
        match self {
            Progress::Finished(f) => f,
            Progress::Unfinished(_) => panic!("called `Progress::unwrap_finished()` on an `Unfinished` value"),
        }
    }

    pub fn as_ref(&self) -> Progress<&U, &F> {
        match self {
            Progress::Unfinished(u) => Progress::Unfinished(u),
            Progress::Finished(f) => Progress::Finished(f),
        }
    }

    pub fn as_mut(&mut self) -> Progress<&mut U, &mut F> {
        match self {
            Progress::Unfinished(u) => Progress::Unfinished(u),
            Progress::Finished(f) => Progress::Finished(f),
        }
    }

    pub fn unfinished_as_ref(&self) -> Option<&U> {
        match self {
            Progress::Unfinished(u) => Some(u),
            Progress::Finished(_) => None,
        }
    }

    pub fn unfinished_as_mut(&mut self) -> Option<&mut U> {
        match self {
            Progress::Unfinished(u) => Some(u),
            Progress::Finished(_) => None,
        }
    }

    pub fn finished_as_ref(&self) -> Option<&F> {
        match self {
            Progress::Finished(f) => Some(f),
            Progress::Unfinished(_) => None,
        }
    }

    pub fn finished_as_mut(&mut self) -> Option<&mut F> {
        match self {
            Progress::Finished(f) => Some(f),
            Progress::Unfinished(_) => None,
        }
    }
}
