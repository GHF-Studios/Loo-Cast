use std::sync::{Arc, RwLock};

pub trait PathSegment: Clone + Eq + PartialEq + std::hash::Hash {
    type Inner<'a> where Self: 'a;

    fn inner<'a>(&'a self) -> Self::Inner<'a>;
    fn type_name(&self) -> &'static str;
}

pub trait StorageMode {
    type View: ViewMode<Self>;

    fn view(&self, mode: &Self::View) -> Arc<RwLock<Self::View>>;
}

pub trait ViewMode<S: StorageMode> {
    type ParentStorage: StorageMode;
    type ChildKey;
    type ChildStorage: StorageMode;

    fn children(view: &S) -> Vec<(Self::ChildKey, Arc<RwLock<Self::ChildStorage>>)>;
}
