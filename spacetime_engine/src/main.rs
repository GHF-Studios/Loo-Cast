// Crates
extern crate spacetime_engine;

// Modules

// Local imports

// Internal imports
use spacetime_engine::*;
use spacetime_engine::kernel::manager::*;
use spacetime_engine::kernel::*;
use spacetime_engine::system::*;

// External imports
use bevy::log::*;
use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::asset::AssetPlugin;
use lazy_static::*;
use std::sync::{Arc, Mutex};
use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;
use libloading::*;
use mlua::*;

// Static variables
lazy_static! {
    pub static ref MAIN_MANAGER: Arc<Mutex<MainManager>> = Arc::new(Mutex::new(MainManager::new()));
}

// Constant variables

// Types

// Enums

// Structs
pub struct MainManager {
    manager_state: ManagerState,
    lua_environment_thread_handle: Option<std::thread::JoinHandle<()>>,
}

// Implementations
impl Manager for MainManager {
    fn initialize(&mut self) -> std::result::Result<(), ManagerInitializeError> {
        info!("Initializing spacetime engine main module...");

        match self.manager_state {
            ManagerState::Created => {},
            ManagerState::Initialized => {
                return Err(ManagerInitializeError::ManagerAlreadyInitialized);
            },
            ManagerState::Finalized => {
                return Err(ManagerInitializeError::ManagerAlreadyFinalized);
            },
        }

        debug!("Locking spacetime engine module manager mutexes...");

        let kernel_manager = KERNEL_MANAGER.clone();
        let mut kernel_manager = match kernel_manager.lock() {
            Ok(kernel_manager) => {
                trace!("Locked kernel manager mutex.");
                kernel_manager
            }
            Err(err) => {
                panic!("Failed to lock kernel manager mutex! Error: {:?}", err);
            }
        };
        let system_manager = SYSTEM_MANAGER.clone();
        let mut system_manager = match system_manager.lock() {
            Ok(system_manager) => {
                trace!("Locked system manager mutex.");
                system_manager
            }
            Err(err) => {
                panic!("Failed to lock system manager mutex! Error: {:?}", err);
            }
        };

        debug!("Locked spacetime engine module manager mutexes.");

        info!("Initializing spacetime engine modules...");

        match kernel_manager.initialize() {
            Ok(_) => {
                debug!("Initialized kernel main module.");
                drop(kernel_manager);
            }
            Err(err) => {
                panic!("Failed to initialize kernel main module! Error: {:?}", err);
            }
        };
        match system_manager.initialize() {
            Ok(_) => {
                debug!("Initialized system main module.");
                drop(system_manager);
            }
            Err(err) => {
                panic!("Failed to initialize system main module! Error: {:?}", err);
            }
        };

        info!("Initialized spacetime engine modules.");

        self.manager_state = ManagerState::Initialized;

        info!("Initialized spacetime engine main module.");

        Ok(())
    }

    fn finalize(&mut self) -> std::result::Result<(), ManagerFinalizeError> {
        info!("Finalizing spacetime engine main module...");

        match self.manager_state {
            ManagerState::Created => {
                return Err(ManagerFinalizeError::ManagerNotInitialized);
            },
            ManagerState::Initialized => {},
            ManagerState::Finalized => {
                return Err(ManagerFinalizeError::ManagerAlreadyFinalized);
            },
        }

        debug!("Locking spacetime engine module manager mutexes...");

        let system_manager = SYSTEM_MANAGER.clone();
        let mut system_manager = match system_manager.lock() {
            Ok(system_manager) => {
                trace!("Locked system manager mutex.");
                system_manager
            }
            Err(err) => {
                panic!("Failed to lock system manager mutex! Error: {:?}", err);
            }
        };
        let kernel_manager = KERNEL_MANAGER.clone();
        let mut kernel_manager = match kernel_manager.lock() {
            Ok(kernel_manager) => {
                trace!("Locked kernel manager mutex.");
                kernel_manager
            }
            Err(err) => {
                panic!("Failed to lock kernel manager mutex! Error: {:?}", err);
            }
        };

        debug!("Locked spacetime engine module manager mutexes.");

        info!("Finalizing spacetime engine modules...");

        match system_manager.finalize() {
            Ok(_) => {
                debug!("Finalized system main module.");
                drop(system_manager);
            }
            Err(err) => {
                panic!("Failed to finalize system! Error: {:?}", err);
            }
        };
        match kernel_manager.finalize() {
            Ok(_) => {
                debug!("Finalized kernel main module.");
                drop(kernel_manager);
            }
            Err(err) => {
                panic!("Failed to finalize kernel! Error: {:?}", err);
            }
        };

        info!("Finalized spacetime engine modules.");

        self.manager_state = ManagerState::Finalized;

        info!("Finalized spacetime engine main module.");

        Ok(())
    }

    fn get_manager_state(&self) -> &ManagerState {
        &self.manager_state
    }
}

impl MainManager {
    fn new() -> Self {
        Self {
            manager_state: ManagerState::Created,
            lua_environment_thread_handle: None,
        }
    }

    fn spacetime_engine_pre_startup() {
        info!("Pre-Starting spacetime engine...");
    
        trace!("Locking spacetime engine main module manager mutex...");
    
        let main_manager = MAIN_MANAGER.clone();
        let mut main_manager = match main_manager.lock() {
            Ok(main_manager) => {
                trace!("Locked spacetime engine main module manager mutex.");
                main_manager
            }
            Err(err) => {
                panic!("Failed to lock spacetime engine main module manager mutex! Error: {:?}", err);
            }
        };
    
        debug!("Initializing spacetime engine main module...");
    
        match main_manager.initialize() {
            Ok(_) => {
                debug!("Initialized spacetime engine main module.");
                drop(main_manager);
            }
            Err(err) => {
                panic!("Failed to initialize spacetime engine main module! Error: {:?}", err);
            }
        };
    
        info!("Pre-Started spacetime engine.");
    }

    fn spacetime_engine_startup() {
        info!("Starting spacetime engine...");

        trace!("Locking spacetime engine main module manager mutex...");
    
        let main_manager = MAIN_MANAGER.clone();
        let mut main_manager = match main_manager.lock() {
            Ok(main_manager) => {
                trace!("Locked spacetime engine main module manager mutex.");
                main_manager
            }
            Err(err) => {
                panic!("Failed to lock spacetime engine main module manager mutex! Error: {:?}", err);
            }
        };
    
        debug!("Spawning lua environment thread...");

        let lua = Lua::new();

        let globals = lua.globals();

        let hello_world_function = lua.create_function(|_, ()| {
            info!("Hello World from Lua!");
            Ok(())
        }).unwrap();

        match globals.set("print", hello_world_function) {
            Ok(_) => {
                debug!("Set 'print' function in lua environment.");
            }
            Err(err) => {
                panic!("Failed to set 'print' function in lua environment: {:?}!", err);
            }
        };


        lua.load(r#"
            print("Hello World!")
        "#).exec().unwrap();

        //main_manager.lua_environment_thread_handle = Some(std::thread::spawn(|| {
        //    
        //}));

        info!("Started spacetime engine.");
    }

    fn spacetime_engine_post_startup() {
        info!("Post-Starting spacetime engine...");

        info!("Post-Started spacetime engine.");
    }
    
    fn spacetime_engine_shutdown(mut exit_events: EventReader<AppExit>) {
        for _ in exit_events.iter() {
            info!("Shutting down spacetime engine...");
    
            trace!("Locking spacetime engine main module manager mutex...");
    
            let main_manager = MAIN_MANAGER.clone();
            let mut main_manager = match main_manager.lock() {
                Ok(main_manager) => {
                    trace!("Locked spacetime engine main module manager mutex.");
                    main_manager
                }
                Err(err) => {
                    panic!("Failed to lock spacetime engine main module manager mutex! Error: {:?}", err);
                }
            };
    
            debug!("Finalizing spacetime engine main module...");
    
            match main_manager.finalize() {
                Ok(_) => {
                    debug!("Finalized spacetime engine main module.");
                    drop(main_manager);
                }
                Err(err) => {
                    panic!("Failed to finalize spacetime engine main module! Error: {:?}", err);
                }
            };
    
            info!("Shut down spacetime engine.");
        }
    }
}

// Module Functions
fn main() {
    let mut app = App::new();

    match find_and_load_mods(&mut app) {
        Ok(_) => {
            println!("Loaded mods.");
        }
        Err(err) => {
            panic!("Failed to load mods: {:?}", err);
        }
    };

    app
        // Startup Systems
        .add_systems(PreStartup, MainManager::spacetime_engine_pre_startup)
        .add_systems(Startup, MainManager::spacetime_engine_startup)
        .add_systems(PostStartup, MainManager::spacetime_engine_post_startup)
        // Update Systems
        .add_systems(Update, MainManager::spacetime_engine_shutdown)
        // Default Bevy Plugins
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    asset_folder: "mods".to_string(),
                    ..default()
                })
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    ..default()
                }),
        )
        // Plugins
        .add_plugins(SystemPlugins)
        // States
        .add_state::<AppState>()
        // Run
        .run();

}

fn find_and_load_mods(app: &mut App) -> std::result::Result<(), Box<dyn Error>> {
    println!("Loading mods...");

    let exe_file_path = match std::env::current_exe() {
        Ok(exe_file_path) => {
            exe_file_path
        }
        Err(err) => {
            panic!("Failed to get current executable path: {:?}", err);
        }
    };

    let mods_folder_path = match exe_file_path.parent() {
        Some(mods_folder_path) => {
            mods_folder_path
        }
        None => {
            panic!("Failed to get application path!");
        }
    };
    let mods_folder_path = mods_folder_path.join("mods");

    println!("Mods folder path: '{:?}'", mods_folder_path);

    let mods_folder_entries = match fs::read_dir(&mods_folder_path) {
        Ok(mods_folder_entries) => {
            mods_folder_entries
        }
        Err(err) => {
            panic!("Failed to read mods folder: {:?}", err);
        }
    };

    for entry in mods_folder_entries {
        let path = match entry {
            Ok(entry) => {
                entry.path()
            }
            Err(err) => {
                panic!("Failed to read mods folder entry: {:?}", err);
            }
        };

        if path.is_dir() {
            println!("Found mod: '{:?}'", path);
            
            let mod_name = match path.file_name() {
                Some(mod_name) => {
                    mod_name
                }
                None => {
                    panic!("Failed to get mod name!");
                }
            };
            let mod_name = match mod_name.to_str() {
                Some(mod_name) => {
                    mod_name.to_owned()
                }
                None => {
                    panic!("Failed to convert mod name to string!");
                }
            };

            let mod_folders = match find_mod_folders(&path) {
                Ok(mod_folders) => {
                    mod_folders
                }
                Err(err) => {
                    panic!("Failed to find mod folders: {:?}", err);
                }
            };

            let mod_files = match find_mod_files(&path) {
                Ok(mod_files) => {
                    mod_files
                }
                Err(err) => {
                    panic!("Failed to find mod files: {:?}", err);
                }
            };

            let mod_dll_file: PathBuf = path.join(mod_name + ".dll");

            println!("Loading mod: '{:?}'", path);

            match load_mod(&mod_dll_file, app) {
                Ok(_) => {
                    println!("Loaded mod '{:?}'.", path);
                }
                Err(err) => {
                    panic!("Failed to load mod '{:?}': {:?}", path, err);
                }
            
            };
        }
    }

    println!("Loaded mods.");

    Ok(())
}

fn find_mod_folders(dir: &Path) -> std::result::Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut mod_folders = Vec::new();

    let entries = match fs::read_dir(&dir) {
        Ok(entries) => {
            entries
        }
        Err(err) => {
            panic!("Failed to read mods folder: {:?}", err);
        }
    };

    for entry in entries {
        let path = match entry {
            Ok(entry) => {
                entry.path()
            }
            Err(err) => {
                panic!("Failed to read mods folder entry: {:?}", err);
            }
        };

        if path.is_dir() {
            mod_folders.push(path);
        }
    }

    Ok(mod_folders)
}

fn find_mod_files(dir: &Path) -> std::result::Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut mod_files = Vec::new();

    let entries = match fs::read_dir(&dir) {
        Ok(entries) => {
            entries
        }
        Err(err) => {
            panic!("Failed to read mods folder: {:?}", err);
        }
    };

    for entry in entries {
        let path = match entry {
            Ok(entry) => {
                entry.path()
            }
            Err(err) => {
                panic!("Failed to read mods folder entry: {:?}", err);
            }
        };

        if path.is_file() && path.file_name().unwrap() == "mod.dll" {
            mod_files.push(path);
        }
    }

    Ok(mod_files)
}

fn load_mod(dll_path: &Path, app: &mut App) -> std::result::Result<(), Box<dyn Error>> {
    println!("Loading mod '{:?}'...", dll_path);

    unsafe {
        println!("Creating library from '{:?}'...", dll_path);
        let lib = Library::new(dll_path)?;

        println!("Getting 'get_mod' symbol from '{:?}'...", dll_path);
        let get_mod: Symbol<unsafe fn()-> *mut dyn Mod> = lib.get(b"get_mod")?;

        println!("Calling 'get_mod' symbol from '{:?}'...", dll_path);
        let spacetime_engine_mod = get_mod();

        println!("Registering mod '{:?}'...", dll_path);
        (*spacetime_engine_mod).register_mod(app);

        println!("Registered mod '{:?}'.", dll_path);
        Ok(())
    }
}

