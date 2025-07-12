use crate::typed_dag::*;
use crate::typed_dag::Xor2;

#[derive(Clone, Debug)]
pub struct DiskInfo(pub &'static str);

#[derive(Clone, Debug)]
pub struct FolderInfo(pub &'static str);

#[derive(Clone, Debug)]
pub struct FileInfo(pub &'static str);

//
// == Disk Node ==
//

#[derive(Clone, Debug)]
pub struct Disk {
    pub payload: DiskInfo,
    pub children: (Vec<Folder>, Vec<File>),
}

impl DagNode for Disk {
    type Payload = DiskInfo;
    type Children = (Vec<Folder>, Vec<File>);

    fn payload(&self) -> &Self::Payload { &self.payload }
    fn payload_mut(&mut self) -> &mut Self::Payload { &mut self.payload }
    fn children(&self) -> &Self::Children { &self.children }
    fn children_mut(&mut self) -> &mut Self::Children { &mut self.children }
}

impl ReversibleDagNode for Disk {
    fn destructure(self) -> (Self::Payload, Self::Children) {
        (self.payload, self.children)
    }

    fn reconstruct(payload: Self::Payload, children: Self::Children) -> Self {
        Self { payload, children }
    }
}

//
// == Folder Node ==
//

#[derive(Clone, Debug)]
pub struct Folder {
    pub payload: FolderInfo,
    pub children: (Vec<Folder>, Vec<File>),
}

impl DagNode for Folder {
    type Payload = FolderInfo;
    type Children = (Vec<Folder>, Vec<File>);

    fn payload(&self) -> &Self::Payload { &self.payload }
    fn payload_mut(&mut self) -> &mut Self::Payload { &mut self.payload }
    fn children(&self) -> &Self::Children { &self.children }
    fn children_mut(&mut self) -> &mut Self::Children { &mut self.children }
}

impl ReversibleDagNode for Folder {
    fn destructure(self) -> (Self::Payload, Self::Children) {
        (self.payload, self.children)
    }

    fn reconstruct(payload: Self::Payload, children: Self::Children) -> Self {
        Self { payload, children }
    }
}

//
// == File Node ==
//

#[derive(Clone, Debug)]
pub struct File {
    pub payload: FileInfo,
    pub _children: ()
}

impl DagNode for File {
    type Payload = FileInfo;
    type Children = ();

    fn payload(&self) -> &Self::Payload { &self.payload }
    fn payload_mut(&mut self) -> &mut Self::Payload { &mut self.payload }
    fn children(&self) -> &Self::Children { &() }
    fn children_mut(&mut self) -> &mut Self::Children { &mut self._children }
}

impl ReversibleDagNode for File {
    fn destructure(self) -> (Self::Payload, Self::Children) {
        (self.payload, ())
    }

    fn reconstruct(payload: Self::Payload, _children: Self::Children) -> Self {
        Self { payload, _children }
    }
}

//
// == DAG Construction Helper ==
//

pub fn make_example_disk() -> Disk {
    let file = File { payload: FileInfo("readme.md"), _children: () };
    let file2 = File { payload: FileInfo("log.txt"), _children: () };
    let file3 = File { payload: FileInfo("output.log"), _children: () };

    let folder_a = Folder {
        payload: FolderInfo("bin"),
        children: (vec![], vec![file3.clone()]),
    };

    let folder_b = Folder {
        payload: FolderInfo("src"),
        children: (vec![folder_a.clone()], vec![file.clone(), file2.clone()]),
    };

    let disk = Disk {
        payload: DiskInfo("/dev/sda1"),
        children: (vec![folder_a, folder_b], vec![file]),
    };

    disk
}
