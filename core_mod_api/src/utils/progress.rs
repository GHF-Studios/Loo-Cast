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
}