use core_engine_macros::DagNode;

#[derive(DagNode)]
#[dag(
    parent = Folder,
    children = (File, Folder)
)]
pub struct Folder {
    pub name: String,
}

#[derive(DagNode)]
#[dag(
    parent = Folder,
    children = ()
)]
pub struct File {
    pub contents: String,
}

pub fn test_example() {
    let root = Folder::new(
        "src".into(),
        (
            File::new("main.rs".into(), ()),
            Folder::new("assets".into(), (...))
        )
    );
}