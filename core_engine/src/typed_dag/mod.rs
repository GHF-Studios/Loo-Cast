pub mod example;

pub trait DagNode {
    type Parent: DagNode;
    type Children: DagChildren;
    type Payload;

    fn payload(&self) -> &Self::Payload;
    fn children(&self) -> &Self::Children;
}

pub trait DagChildren {}

impl DagChildren for () {}
impl<T1: DagNode> DagChildren for (T1,) {}
impl<T1: DagNode, T2: DagNode> DagChildren for (T1, T2) {}
// Add more as needed

pub trait VisitChildren<V> {
    fn visit_children(&self, visitor: &mut V);
}

impl<V> VisitChildren<V> for () {
    fn visit_children(&self, _visitor: &mut V) {}
}

impl<C1, V> VisitChildren<V> for (C1,)
where
    C1: DagNode,
    V: DagVisitor<C1>,
    C1::Children: VisitChildren<V>,
{
    fn visit_children(&self, visitor: &mut V) {
        visit_dag(&self.0, visitor);
    }
}

impl<C1, C2, V> VisitChildren<V> for (C1, C2)
where
    C1: DagNode,
    C2: DagNode,
    V: DagVisitor<C1> + DagVisitor<C2>,
    C1::Children: VisitChildren<V>,
    C2::Children: VisitChildren<V>,
{
    fn visit_children(&self, visitor: &mut V) {
        visit_dag(&self.0, visitor);
        visit_dag(&self.1, visitor);
    }
}

pub trait DagVisitor<Node: DagNode> {
    fn visit(&mut self, node: &Node);
}

pub fn visit_dag<Node, V>(node: &Node, visitor: &mut V)
where
    Node: DagNode,
    V: DagVisitor<Node>,
    Node::Children: VisitChildren<V>,
{
    visitor.visit(node);
    node.children().visit_children(visitor);
}

pub trait ConstructNode: DagNode + Sized {
    fn new(payload: Self::Payload, children: Self::Children) -> Self;
}

pub trait WithChildren: DagNode {
    fn with_children(&self, children: Self::Children) -> Self;
}
