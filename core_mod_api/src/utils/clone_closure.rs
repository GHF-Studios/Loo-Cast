use std::marker::PhantomData;

pub trait ApplyCloneClosure<M, A, R> {
    fn apply(&mut self, moved: M, args: A) -> R;
}
impl<F, R> ApplyCloneClosure<(), (), R> for F
where
    F: FnMut() -> R,
{
    fn apply(&mut self, _: (), _: ()) -> R {
        (self)()
    }
}
impl<F, A1, R> ApplyCloneClosure<(), (A1,), R> for F
where
    F: FnMut(A1) -> R,
{
    fn apply(&mut self, _: (), (a1,): (A1,)) -> R {
        (self)(a1)
    }
}
impl<F, A1, A2, R> ApplyCloneClosure<(), (A1, A2), R> for F
where
    F: FnMut(A1, A2) -> R,
{
    fn apply(&mut self, _: (), (a1, a2): (A1, A2)) -> R {
        (self)(a1, a2)
    }
}
impl<F, M1, R> ApplyCloneClosure<(M1,), (), R> for F
where
    F: FnMut(M1) -> R,
{
    fn apply(&mut self, (m1,): (M1,), _: ()) -> R {
        (self)(m1)
    }
}
impl<F, M1, A1, R> ApplyCloneClosure<(M1,), (A1,), R> for F
where
    F: FnMut(M1, A1) -> R,
{
    fn apply(&mut self, (m1,): (M1,), (a1,): (A1,)) -> R {
        (self)(m1, a1)
    }
}
impl<F, M1, A1, A2, R> ApplyCloneClosure<(M1,), (A1, A2), R> for F
where
    F: FnMut(M1, A1, A2) -> R,
{
    fn apply(&mut self, (m1,): (M1,), (a1, a2): (A1, A2)) -> R {
        (self)(m1, a1, a2)
    }
}
impl<F, M1, M2, R> ApplyCloneClosure<(M1, M2), (), R> for F
where
    F: FnMut(M1, M2) -> R,
{
    fn apply(&mut self, (m1, m2): (M1, M2), _: ()) -> R {
        (self)(m1, m2)
    }
}
impl<F, M1, M2, A1, R> ApplyCloneClosure<(M1, M2), (A1,), R> for F
where
    F: FnMut(M1, M2, A1) -> R,
{
    fn apply(&mut self, (m1, m2): (M1, M2), (a1,): (A1,)) -> R {
        (self)(m1, m2, a1)
    }
}
impl<F, M1, M2, A1, A2, R> ApplyCloneClosure<(M1, M2), (A1, A2), R> for F
where
    F: FnMut(M1, M2, A1, A2) -> R,
{
    fn apply(&mut self, (m1, m2): (M1, M2), (a1, a2): (A1, A2)) -> R {
        (self)(m1, m2, a1, a2)
    }
}

pub struct CloneClosure<M, A, R, F> 
where 
    M: Clone,
    F: Clone + FnMut(M, A) -> R
{
    moved: M,
    phantom_args: PhantomData<A>,
    phantom_r: PhantomData<R>,
    func: F,
}
impl<M, A, R, F> Clone for CloneClosure<M, A, R, F>
where 
    M: Clone,
    F: Clone + FnMut(M, A) -> R
{
    fn clone(&self) -> Self {
        Self::new(self.moved.clone(), self.func.clone())
    }
}
impl<M, A, R, F> CloneClosure<M, A, R, F>
where 
    M: Clone,
    F: Clone + FnMut(M, A) -> R
{
    pub const fn new(moved: M, func: F) -> Self {
        Self { moved, phantom_args: PhantomData, phantom_r: PhantomData, func }
    }

    pub fn apply(
        &mut self,
        args: A,
    ) -> R
    {
        self.func.apply((self.moved.clone(),), (args,))
    }
}
impl<M, A, R, F> FnOnce<(A,)> for CloneClosure<M, A, R, F>
where 
    M: Clone,
    F: Clone + FnMut(M, A) -> R
{
    type Output = R;

    extern "rust-call" fn call_once(mut self, args: (A,)) -> Self::Output {
        self.apply(args.0)
    }
}
impl<M, A, R, F> FnMut<(A,)> for CloneClosure<M, A, R, F> 
where 
    M: Clone,
    F: Clone + FnMut(M, A) -> R
{
    extern "rust-call" fn call_mut(&mut self, args: (A,)) -> Self::Output {
        self.apply(args.0)
    }
}