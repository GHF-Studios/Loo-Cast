//use spacetime_engine_derive::{define_commands_module, define_assets_module};
//use serde::{Deserialize, Serialize};
//
//pub struct TestAssets {}
//
//impl TestAssets {
//    pub fn load_file(
//        file_path: std::path::PathBuf,
//        data_id: u64,
//        file_registry: &FileRegistry,
//        data_registry: &DataRegistry,
//    ) -> Result<(), LoadFileCommandError> {
//        let input = LoadFileCommandInput {
//            file_path,
//            data_id,
//            file_registry,
//            data_registry,
//        };
//
//        match LoadFileCommand::execute(input) {
//            Ok(_) => {
//                Ok(())
//            },
//            Err(error) => {
//                Err(error)
//            },
//        }
//    }
//
//}
//
//pub struct PlayerStatsAsset {
//    file_path: PathBuf,
//    data_id: u64,
//}
//
//impl PlayerStatsAsset {
//    pub fn new(file_path: PathBuf, data_id: u64) -> Self {
//        Self {
//            file_path,
//            data_id,
//        }
//    }
//
//    pub fn deserialize_asset(
//        &self, 
//        file_registry: &mut TestFilesRegistry, 
//        data_registry: &mut TestDatasRegistry
//    ) -> Result<PlayerStatsData, PlayerStatsFileDeserializeError> {
//    }
//
//    pub fn serialize_asset(
//        &self,
//        file_registry: &mut TestFilesRegistry,
//        data_registry: &mut TestDatasRegistry,
//    ) -> Result<PlayerStatsFile, PlayerStatsDataSerializeError> {
//    }
//}
//
//impl Asset for PlayerStatsAsset {
//    
//}
//
//pub struct PlayerStatsFile(crate::kernel::files::BinaryFile);
//
//#[derive(Serialize, Deserialize)]
//pub struct PlayerStatsData {
//    pub id: u64,
//    pub name: String,
//    pub high_score: u32,
//}
//
//pub enum PlayerStatsFileDeserializeError {
//    FileReadError,
//    DeserializeError,
//    DataWriteError,
//}
//
//pub enum PlayerStatsDataSerializeError {
//    DataReadError,
//    SerializeError,
//    FileWriteError,
//}
//
//define_assets_module! {
//    Test {
//        module_path: crate::kernel::commands,
//        assets: [
//            PlayerStats {
//                File {
//                    type_path: crate::kernel::files::Binary
//                },
//                Data {
//                    name: String,
//                    high_score: u32,
//                },
//            },
//        ]
//    }
//}
//
//define_commands_module! {
//    Asset {
//        module_path: crate::kernel::assets,
//        commands: [
//            DeserializeFile {
//                Input {
//                    file_path: std::path::PathBuf,
//                    data_id: u64,
//                },
//                Output {
//                },
//                Error {
//                    FileReadError,
//                    DeserializeError,
//                    DataWriteError,
//                },
//                Code |file_path, data_id| -> Result<Output, Error> {
//                    let file_handle = std::fs::File::open(file_path).map_err(|error| Error::FileReadError)?;
//                    let file = BinaryFile {
//                        file_handle,
//                    };
//                }
//            },
//            SerializeData {
//                Input {
//                    file_path: std::path::PathBuf,
//                    data_id: u64,
//                },
//                Output {
//                },
//                Error {
//                    DataReadError,
//                    SerializeError,
//                    FileWriteError,
//                },
//                Code |file_path, data_id| -> Result<Output, Error> {
//                }
//            }
//        ]
//    }
//}