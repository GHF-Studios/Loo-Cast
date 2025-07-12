pub mod example;

// === Core DAG Traits ===

pub trait DagNode: Sized {
    type Children: DagChildren;
    type Payload;

    fn payload(&self) -> &Self::Payload;
    fn payload_mut(&mut self) -> &mut Self::Payload;
    fn children(&self) -> &Self::Children;
    fn children_mut(&mut self) -> &mut Self::Children;
}

pub trait ReversibleDagNode: DagNode {
    fn destructure(self) -> (Self::Payload, Self::Children);
    fn reconstruct(payload: Self::Payload, children: Self::Children) -> Self;
}

// === Blanket Unit Support for Root/Empty Nodes ===

impl DagNode for () {
    type Children = ();
    type Payload = ();

    fn payload(&self) -> &Self::Payload { &() }
    fn payload_mut(&mut self) -> &mut Self::Payload { self }
    fn children(&self) -> &Self::Children { &() }
    fn children_mut(&mut self) -> &mut Self::Children { self }
}

impl ReversibleDagNode for () {
    fn destructure(self) -> (Self::Payload, Self::Children) { ((), ()) }
    fn reconstruct(_payload: Self::Payload, _children: Self::Children) -> Self {}
}

// === Children Tuple Composition ===

pub trait DagChildren {}
impl DagChildren for () {}
impl<T: DagNode> DagChildren for (Vec<T>,) {}
impl<T1: DagNode, T2: DagNode> DagChildren for (Vec<T1>, Vec<T2>) {}

// === XOR Utility Enum for Parent Relationships ===

#[derive(Clone)]
pub enum Xor2<A, B> {
    A(A),
    B(B),
}

impl<A, B> Xor2<A, B> {
    pub fn map<T, FA, FB>(self, fa: FA, fb: FB) -> T
    where
        FA: FnOnce(A) -> T,
        FB: FnOnce(B) -> T,
    {
        match self {
            Xor2::A(a) => fa(a),
            Xor2::B(b) => fb(b),
        }
    }

    pub fn as_ref(&self) -> Xor2<&A, &B> {
        match self {
            Xor2::A(a) => Xor2::A(a),
            Xor2::B(b) => Xor2::B(b),
        }
    }
}

// === DFS VISITOR ===

pub fn dfs_preorder<N: ReversibleDagNode, F: FnMut(&N::Payload)>(node: &N, visit: &mut F)
where
    N::Children: DfsVisit<F>,
{
    visit(node.payload());
    node.children().dfs_visit(visit);
}

pub trait DfsVisit<F> {
    fn dfs_visit(&self, f: &mut F);
}

impl<F, N: ReversibleDagNode> DfsVisit<F> for Vec<N>
where
    N::Children: DfsVisit<F>,
    F: FnMut(&N::Payload),
{
    fn dfs_visit(&self, f: &mut F) {
        for child in self {
            dfs_preorder(child, f);
        }
    }
}

impl<F, T1, T2> DfsVisit<F> for (Vec<T1>, Vec<T2>)
where
    T1: ReversibleDagNode,
    T2: ReversibleDagNode,
    T1::Children: DfsVisit<F>,
    T2::Children: DfsVisit<F>,
    F: FnMut(&T1::Payload) + FnMut(&T2::Payload),
{
    fn dfs_visit(&self, f: &mut F) {
        self.0.dfs_visit(f);
        self.1.dfs_visit(f);
    }
}

impl<F> DfsVisit<F> for () {
    fn dfs_visit(&self, _f: &mut F) {}
}
