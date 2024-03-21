//use spacetime_engine_derive::define_files_module;
//
//pub struct BinaryFile {
//    pub file_handle: std::fs::File,
//}
//
//impl File for BinaryFile {
//    
//}
//
//define_files_module! {
//    Test {
//        module_path: crate::kernel::files,
//        files: [
//            Binary(Vec<u8>),
//            Plaintext(String),
//            XML(String),
//            JSON(String),
//        ]
//    }
//}