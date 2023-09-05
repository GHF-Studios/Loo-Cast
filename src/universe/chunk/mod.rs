use super::components::Chunk;
use super::LocalChunkPosition;

use crate::noise::*;

use bevy::prelude::*;
use image::*;
use std::collections::*;
use std::fs::*;
use std::io::Write;
use std::path::Path;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Callback = Box<dyn FnOnce() + Send>;

#[derive(PartialEq, Clone)]
pub enum ChunkState {
    QueuedForGeneration,
    Generating,
    Generated,
    QueuedForLoading,
    Loading,
    Loaded,
    QueuedForSpawning,
    Spawning,
    Spawned,
    QueuedForDespawning,
    Despawning,
    Despawned,
    QueuedForUnloading,
    Unloading,
    Unloaded,
}

struct ChunkInfo {
    state: ChunkState,
    position: LocalChunkPosition,
}

struct CallbackInfo {
    callback: Callback,
    cancelled: bool,
}

pub enum ChunkOperation {
    Generate,
    Load,
    Spawn,
    Despawn,
    Unload,
}

#[derive(Resource)]
pub struct ChunkManager {
    chunks_folder_path: String,
    chunk_info_map: Arc<Mutex<HashMap<LocalChunkPosition, ChunkInfo>>>,
    chunk_map: Arc<Mutex<HashMap<LocalChunkPosition, Chunk>>>,

    generate_queue: Arc<Mutex<VecDeque<LocalChunkPosition>>>,
    load_queue: Arc<Mutex<VecDeque<LocalChunkPosition>>>,
    spawn_queue: Arc<Mutex<VecDeque<LocalChunkPosition>>>,
    despawn_queue: Arc<Mutex<VecDeque<LocalChunkPosition>>>,
    unload_queue: Arc<Mutex<VecDeque<LocalChunkPosition>>>,

    generate_callbacks: Arc<Mutex<HashMap<LocalChunkPosition, CallbackInfo>>>,
    load_callbacks: Arc<Mutex<HashMap<LocalChunkPosition, CallbackInfo>>>,
    spawn_callbacks: Arc<Mutex<HashMap<LocalChunkPosition, CallbackInfo>>>,
    despawn_callbacks: Arc<Mutex<HashMap<LocalChunkPosition, CallbackInfo>>>,
    unload_callbacks: Arc<Mutex<HashMap<LocalChunkPosition, CallbackInfo>>>,

    tx_enqueue_generate: Arc<Mutex<mpsc::Sender<(LocalChunkPosition, Callback)>>>,
    tx_enqueue_load: Arc<Mutex<mpsc::Sender<(LocalChunkPosition, Callback)>>>,
    tx_enqueue_spawn: Arc<Mutex<mpsc::Sender<(LocalChunkPosition, Callback)>>>,
    tx_enqueue_despawn: Arc<Mutex<mpsc::Sender<(LocalChunkPosition, Callback)>>>,
    tx_enqueue_unload: Arc<Mutex<mpsc::Sender<(LocalChunkPosition, Callback)>>>,

    tx_process_generate: Arc<Mutex<mpsc::Sender<usize>>>,
    tx_process_load: Arc<Mutex<mpsc::Sender<usize>>>,
    tx_process_spawn: Arc<Mutex<mpsc::Sender<usize>>>,
    tx_process_despawn: Arc<Mutex<mpsc::Sender<usize>>>,
    tx_process_unload: Arc<Mutex<mpsc::Sender<usize>>>,
}

// Actually call the generate/load/spawn/despawn/unload functions
// Instead of calling spawn/despawn, request the main thread to enqueue those functions' calls, which it executes every frame update

struct ChunkManagerInitializationData {
    rx_enqueue_generate: mpsc::Receiver<(LocalChunkPosition, Callback)>,
    rx_enqueue_load: mpsc::Receiver<(LocalChunkPosition, Callback)>,
    rx_enqueue_spawn: mpsc::Receiver<(LocalChunkPosition, Callback)>,
    rx_enqueue_despawn: mpsc::Receiver<(LocalChunkPosition, Callback)>,
    rx_enqueue_unload: mpsc::Receiver<(LocalChunkPosition, Callback)>,

    rx_process_generate: mpsc::Receiver<usize>,
    rx_process_load: mpsc::Receiver<usize>,
    rx_process_spawn: mpsc::Receiver<usize>,
    rx_process_despawn: mpsc::Receiver<usize>,
    rx_process_unload: mpsc::Receiver<usize>,
}

impl ChunkManager {
    pub fn new(chunks_folder_path: String) -> Self {
        let (instance, initialization_data) = Self::create_instance(chunks_folder_path);
        instance.initialize_instance(initialization_data);
        instance
    }

    pub fn enqueue_generate_operation(&self, position: LocalChunkPosition, callback: Callback) -> Result<(), String> {
        let info_map = self.chunk_info_map.lock().unwrap();
        if info_map.contains_key(&position)
        {
            return Err(format!("Chunk with position {:?} already exists", position));
        } else {
            self.tx_enqueue_generate
                .lock()
                .unwrap()
                .send((position, callback))
                .unwrap();
            return Ok(());
        }
    }

    pub fn enqueue_load_operation(&self, position: LocalChunkPosition, callback: Callback) -> Result<(), String> {
        let mut chunk_info_map = self.chunk_info_map.lock().unwrap();
        let mut unload_queue = self.unload_queue.lock().unwrap();
        let mut unload_callbacks = self.unload_callbacks.lock().unwrap();

        if !chunk_info_map.contains_key(&position)
        {
            return Err(format!("Chunk with position {:?} does not exist", position));
        } else {
            match chunk_info_map.get(&position).unwrap().state {
                ChunkState::Generated => {
                    self.tx_enqueue_load
                        .lock()
                        .unwrap()
                        .send((position, callback))
                        .unwrap();
                    return Ok(());
                },
                ChunkState::QueuedForUnloading => {
                    chunk_info_map.entry(position).and_modify(|chunk_info| {
                        chunk_info.state = ChunkState::Loaded;
                    });
                    unload_queue.retain(|&queued_position| queued_position != position);
                    unload_callbacks.remove(&position);
                    return Ok(());
                },
                ChunkState::Unloading => {
                    unload_callbacks.entry(position).and_modify(|callback_info| {
                        callback_info.cancelled = true;
                    });
                    self.tx_enqueue_load
                        .lock()
                        .unwrap()
                        .send((position, callback))
                        .unwrap();
                    return Ok(());
                },
                ChunkState::Unloaded => {
                    self.tx_enqueue_load
                        .lock()
                        .unwrap()
                        .send((position, callback))
                        .unwrap();
                    return Ok(());
                },
                _ => {
                    return Err(format!("Chunk with position {:?} is not in a state which allows it to be loaded. It can only be generated, queued for unloading, unloading or unloaded, when trying to load it", position));
                }
            }
        }
    }

    pub fn enqueue_spawn_operation(&self, position: LocalChunkPosition, callback: Callback) -> Result<(), String> {
        let mut chunk_info_map = self.chunk_info_map.lock().unwrap();
        let mut despawn_queue = self.despawn_queue.lock().unwrap();
        let mut despawn_callbacks = self.despawn_callbacks.lock().unwrap();

        if !chunk_info_map.contains_key(&position)
        {
            return Err(format!("Chunk with position {:?} does not exist", position));
        } else {
            match chunk_info_map.get(&position).unwrap().state {
                ChunkState::Loaded => {
                    self.tx_enqueue_load
                        .lock()
                        .unwrap()
                        .send((position, callback))
                        .unwrap();
                    return Ok(());
                },
                ChunkState::QueuedForDespawning => {
                    chunk_info_map.entry(position).and_modify(|chunk_info| {
                        chunk_info.state = ChunkState::Spawned;
                    });
                    despawn_queue.retain(|&queued_position| queued_position != position);
                    despawn_callbacks.remove(&position);
                    return Ok(());
                },
                ChunkState::Despawning => {
                    despawn_callbacks.entry(position).and_modify(|callback_info| {
                        callback_info.cancelled = true;
                    });
                    self.tx_enqueue_load
                        .lock()
                        .unwrap()
                        .send((position, callback))
                        .unwrap();
                    return Ok(());
                },
                ChunkState::Despawned => {
                    self.tx_enqueue_load
                        .lock()
                        .unwrap()
                        .send((position, callback))
                        .unwrap();
                    return Ok(());
                },
                _ => {
                    return Err(format!("Chunk with position {:?} is not in a state which allows it to be spawned. It can only be loaded, queued for despawning, despawning or despawned, when trying to spawn it", position));
                }
            }
        }
    }

    pub fn enqueue_despawn_operation(&self, position: LocalChunkPosition, callback: Callback) -> Result<(), String> {
        let mut chunk_info_map = self.chunk_info_map.lock().unwrap();
        let mut spawn_queue = self.spawn_queue.lock().unwrap();
        let mut spawn_callbacks = self.spawn_callbacks.lock().unwrap();

        if !chunk_info_map.contains_key(&position)
        {
            return Err(format!("Chunk with position {:?} does not exist", position));
        } else {
            match chunk_info_map.get(&position).unwrap().state {
                ChunkState::QueuedForSpawning => {
                    chunk_info_map.entry(position).and_modify(|chunk_info| {
                        chunk_info.state = ChunkState::Despawned;
                    });
                    spawn_queue.retain(|&queued_position| queued_position != position);
                    spawn_callbacks.remove(&position);
                    return Ok(());
                },
                ChunkState::Spawning => {
                    spawn_callbacks.entry(position).and_modify(|callback_info| {
                        callback_info.cancelled = true;
                    });
                    self.tx_enqueue_despawn
                        .lock()
                        .unwrap()
                        .send((position, callback))
                        .unwrap();
                    return Ok(());
                },
                ChunkState::Spawned => {
                    self.tx_enqueue_despawn
                        .lock()
                        .unwrap()
                        .send((position, callback))
                        .unwrap();
                    return Ok(());
                },
                _ => {
                    return Err(format!("Chunk with position {:?} is not in a state which allows it to be despawned. It can only be queued for spawning, spawning or spawned, when trying to despawn it", position));
                }
            }
        }
    }

    pub fn enqueue_unload_operation(&self, position: LocalChunkPosition, callback: Callback) -> Result<(), String> {
        let mut chunk_info_map = self.chunk_info_map.lock().unwrap();
        let mut load_queue = self.load_queue.lock().unwrap();
        let mut load_callbacks = self.load_callbacks.lock().unwrap();

        if !chunk_info_map.contains_key(&position)
        {
            return Err(format!("Chunk with position {:?} does not exist", position));
        } else {
            match chunk_info_map.get(&position).unwrap().state {
                ChunkState::Despawned => {
                    self.tx_enqueue_unload
                        .lock()
                        .unwrap()
                        .send((position, callback))
                        .unwrap();
                    return Ok(());
                },
                ChunkState::QueuedForLoading => {
                    chunk_info_map.entry(position).and_modify(|chunk_info| {
                        chunk_info.state = ChunkState::Unloaded;
                    });
                    load_queue.retain(|&queued_position| queued_position != position);
                    load_callbacks.remove(&position);
                    return Ok(());
                },
                ChunkState::Loading => {
                    load_callbacks.entry(position).and_modify(|callback_info| {
                        callback_info.cancelled = true;
                    });
                    self.tx_enqueue_unload
                        .lock()
                        .unwrap()
                        .send((position, callback))
                        .unwrap();
                    return Ok(());
                },
                ChunkState::Loaded => {
                    self.tx_enqueue_unload
                        .lock()
                        .unwrap()
                        .send((position, callback))
                        .unwrap();
                    return Ok(());
                },
                _ => {
                    return Err(format!("Chunk with position {:?} is not in a state which allows it to be unload. It can only be queued for loading, loading, loaded or despawned, when trying to unload it", position));
                }
            }
        }
    }

    pub fn get_chunk_state(&self, position: LocalChunkPosition) -> Option<ChunkState> {
        if let Some(chunk_info) = self.chunk_info_map.lock().unwrap().get(&position) {
            Some(chunk_info.state.clone())
        } else {
            None
        }
    }

    fn create_instance(chunks_folder_path: String) -> (Self, ChunkManagerInitializationData) {
        let (tx_enqueue_generate, rx_enqueue_generate) = mpsc::channel::<(LocalChunkPosition, Callback)>();
        let (tx_enqueue_load, rx_enqueue_load) = mpsc::channel::<(LocalChunkPosition, Callback)>();
        let (tx_enqueue_spawn, rx_enqueue_spawn) = mpsc::channel::<(LocalChunkPosition, Callback)>();
        let (tx_enqueue_despawn, rx_enqueue_despawn) = mpsc::channel::<(LocalChunkPosition, Callback)>();
        let (tx_enqueue_unload, rx_enqueue_unload) = mpsc::channel::<(LocalChunkPosition, Callback)>();

        let (tx_process_generate, rx_process_generate) = mpsc::channel::<usize>();
        let (tx_process_load, rx_process_load) = mpsc::channel::<usize>();
        let (tx_process_spawn, rx_process_spawn) = mpsc::channel::<usize>();
        let (tx_process_despawn, rx_process_despawn) = mpsc::channel::<usize>();
        let (tx_process_unload, rx_process_unload) = mpsc::channel::<usize>();

        let tx_enqueue_generate = Arc::new(Mutex::new(tx_enqueue_generate));
        let tx_enqueue_load = Arc::new(Mutex::new(tx_enqueue_load));
        let tx_enqueue_spawn = Arc::new(Mutex::new(tx_enqueue_spawn));
        let tx_enqueue_despawn = Arc::new(Mutex::new(tx_enqueue_despawn));
        let tx_enqueue_unload = Arc::new(Mutex::new(tx_enqueue_unload));

        let tx_process_generate = Arc::new(Mutex::new(tx_process_generate));
        let tx_process_load = Arc::new(Mutex::new(tx_process_load));
        let tx_process_spawn = Arc::new(Mutex::new(tx_process_spawn));
        let tx_process_despawn = Arc::new(Mutex::new(tx_process_despawn));
        let tx_process_unload = Arc::new(Mutex::new(tx_process_unload));

        let generate_queue = Arc::new(Mutex::new(VecDeque::new()));
        let load_queue = Arc::new(Mutex::new(VecDeque::new()));
        let spawn_queue = Arc::new(Mutex::new(VecDeque::new()));
        let despawn_queue = Arc::new(Mutex::new(VecDeque::new()));
        let unload_queue = Arc::new(Mutex::new(VecDeque::new()));

        (
            Self {
                chunks_folder_path,
                chunk_info_map: Arc::new(Mutex::new(HashMap::new())),
                chunk_map: Arc::new(Mutex::new(HashMap::new())),

                generate_queue,
                load_queue,
                spawn_queue,
                despawn_queue,
                unload_queue,

                generate_callbacks: Arc::new(Mutex::new(HashMap::new())),
                load_callbacks: Arc::new(Mutex::new(HashMap::new())),
                spawn_callbacks: Arc::new(Mutex::new(HashMap::new())),
                despawn_callbacks: Arc::new(Mutex::new(HashMap::new())),
                unload_callbacks: Arc::new(Mutex::new(HashMap::new())),

                tx_enqueue_generate,
                tx_enqueue_load,
                tx_enqueue_spawn,
                tx_enqueue_despawn,
                tx_enqueue_unload,

                tx_process_generate,
                tx_process_load,
                tx_process_spawn,
                tx_process_despawn,
                tx_process_unload,
            },
            ChunkManagerInitializationData {
                rx_enqueue_generate,
                rx_enqueue_load,
                rx_enqueue_spawn,
                rx_enqueue_despawn,
                rx_enqueue_unload,

                rx_process_generate,
                rx_process_load,
                rx_process_spawn,
                rx_process_despawn,
                rx_process_unload,
            },
        )
    }

    fn initialize_instance(&self, initialization_data: ChunkManagerInitializationData) {
        let chunk_info_map = self.chunk_info_map.clone();

        let generate_queue = self.generate_queue.clone();
        let load_queue = self.load_queue.clone();
        let spawn_queue = self.spawn_queue.clone();
        let despawn_queue = self.despawn_queue.clone();
        let unload_queue = self.unload_queue.clone();

        let generate_callbacks = self.generate_callbacks.clone();
        let load_callbacks = self.load_callbacks.clone();
        let spawn_callbacks = self.spawn_callbacks.clone();
        let despawn_callbacks = self.despawn_callbacks.clone();
        let unload_callbacks = self.unload_callbacks.clone();

        thread::spawn(move || loop {
            println!("Started enqueueing generate operations");
            while let Ok((position, callback)) = initialization_data.rx_enqueue_generate.recv() {
                let mut queue = generate_queue.lock().unwrap();
                let mut info_map = chunk_info_map.lock().unwrap();
                let mut callbacks = generate_callbacks.lock().unwrap();

                queue.push_back(position);
                info_map.insert(position, ChunkInfo {
                    state: ChunkState::QueuedForGeneration,
                    position,
                });
                callbacks.insert(
                    position,
                    CallbackInfo {
                        callback,
                        cancelled: false,
                    },
                );
            }
            println!("Finished enqueueing generate operations");
            
            println!("Started enqueueing load operations");
            while let Ok((position, callback)) = initialization_data.rx_enqueue_load.recv() {
                let mut queue = load_queue.lock().unwrap();
                let mut info_map = chunk_info_map.lock().unwrap();
                let mut callbacks = load_callbacks.lock().unwrap();
                
                queue.push_back(position);
                info_map.entry(position).and_modify(|chunk_info| {
                    chunk_info.state = ChunkState::QueuedForLoading;
                });
                callbacks.insert(
                    position,
                    CallbackInfo {
                        callback,
                        cancelled: false,
                    },
                );
            }
            println!("Finished enqueueing load operations");

            println!("Started enqueueing spawn operations");
            while let Ok((position, callback)) = initialization_data.rx_enqueue_spawn.recv() {
                let mut queue = spawn_queue.lock().unwrap();
                let mut info_map = chunk_info_map.lock().unwrap();
                let mut callbacks = spawn_callbacks.lock().unwrap();
                
                queue.push_back(position);
                info_map.entry(position).and_modify(|chunk_info| {
                    chunk_info.state = ChunkState::QueuedForSpawning;
                });
                callbacks.insert(
                    position,
                    CallbackInfo {
                        callback,
                        cancelled: false,
                    },
                );
            }
            println!("Finished enqueueing spawn operations");

            println!("Started enqueueing despawn operations");
            while let Ok((position, callback)) = initialization_data.rx_enqueue_despawn.recv() {
                let mut queue = despawn_queue.lock().unwrap();
                let mut info_map = chunk_info_map.lock().unwrap();
                let mut callbacks = despawn_callbacks.lock().unwrap();
                
                queue.push_back(position);
                info_map.entry(position).and_modify(|chunk_info| {
                    chunk_info.state = ChunkState::QueuedForDespawning;
                });
                callbacks.insert(
                    position,
                    CallbackInfo {
                        callback,
                        cancelled: false,
                    },
                );
            }
            println!("Finished enqueueing despawn operations");

            println!("Started enqueueing unload operations");
            while let Ok((position, callback)) = initialization_data.rx_enqueue_unload.recv() {
                let mut queue = unload_queue.lock().unwrap();
                let mut info_map = chunk_info_map.lock().unwrap();
                let mut callbacks = unload_callbacks.lock().unwrap();
                
                queue.push_back(position);
                info_map.entry(position).and_modify(|chunk_info| {
                    chunk_info.state = ChunkState::QueuedForUnloading;
                });
                callbacks.insert(
                    position,
                    CallbackInfo {
                        callback,
                        cancelled: false,
                    },
                );
            }
            println!("Finished enqueueing unload operations");


            println!("Started processing generate operations");
            while let Some(position) = generate_queue.lock().unwrap().pop_front() {
                chunk_info_map.lock().unwrap().entry(position).and_modify(|chunk_info| {
                    chunk_info.state = ChunkState::Generating;
                });

                println!("Generating chunk at position {:?}", position);

                chunk_info_map.lock().unwrap().entry(position).and_modify(|chunk_info| {
                    chunk_info.state = ChunkState::Generated;
                });

                if let Some(callback_info) = generate_callbacks.lock().unwrap().remove(&position) {
                    if !callback_info.cancelled {
                        (callback_info.callback)();
                    }
                }
            }
            println!("Finished processing generate operations");

            println!("Started processing load operations");
            while let Some(position) = load_queue.lock().unwrap().pop_front() {
                chunk_info_map.lock().unwrap().entry(position).and_modify(|chunk_info| {
                    chunk_info.state = ChunkState::Loading;
                });

                println!("Loading chunk at position {:?}", position);

                chunk_info_map.lock().unwrap().entry(position).and_modify(|chunk_info| {
                    chunk_info.state = ChunkState::Loaded;
                });

                if let Some(callback_info) = load_callbacks.lock().unwrap().remove(&position) {
                    if !callback_info.cancelled {
                        (callback_info.callback)();
                    }
                }
            }
            println!("Finished processing load operations");

            println!("Started processing spawn operations");
            while let Some(position) = spawn_queue.lock().unwrap().pop_front() {
                chunk_info_map.lock().unwrap().entry(position).and_modify(|chunk_info| {
                    chunk_info.state = ChunkState::Spawning;
                });

                println!("Spawning chunk at position {:?}", position);

                chunk_info_map.lock().unwrap().entry(position).and_modify(|chunk_info| {
                    chunk_info.state = ChunkState::Spawned;
                });

                if let Some(callback_info) = spawn_callbacks.lock().unwrap().remove(&position) {
                    if !callback_info.cancelled {
                        (callback_info.callback)();
                    }
                }
            }
            println!("Finished processing spawn operations");

            println!("Started processing despawn operations");
            while let Some(position) = despawn_queue.lock().unwrap().pop_front() {
                chunk_info_map.lock().unwrap().entry(position).and_modify(|chunk_info| {
                    chunk_info.state = ChunkState::Despawning;
                });

                println!("Despawning chunk at position {:?}", position);

                chunk_info_map.lock().unwrap().entry(position).and_modify(|chunk_info| {
                    chunk_info.state = ChunkState::Despawned;
                });

                if let Some(callback_info) = despawn_callbacks.lock().unwrap().remove(&position)
                {
                    if !callback_info.cancelled {
                        (callback_info.callback)();
                    }
                }
            }
            println!("Finished processing despawn operations");

            println!("Started processing unload operations");
            while let Some(position) = unload_queue.lock().unwrap().pop_front() {
                chunk_info_map.lock().unwrap().entry(position).and_modify(|chunk_info| {
                    chunk_info.state = ChunkState::Unloading;
                });

                println!("Unloading chunk at position {:?}", position);
                
                chunk_info_map.lock().unwrap().entry(position).and_modify(|chunk_info| {
                    chunk_info.state = ChunkState::Unloaded;
                });

                if let Some(callback_info) = unload_callbacks.lock().unwrap().remove(&position)
                {
                    if !callback_info.cancelled {
                        (callback_info.callback)();
                    }
                }
            }
            println!("Finished processing unload operations");
        });
    }

    fn generate_chunk(&self, chunk_pos: LocalChunkPosition) {
        let chunk = Chunk {
            pos: chunk_pos,
            scale_level: 0,
            stored_entities: Vec::new(),
        };

        let serialized_chunk: String = serde_json::to_string(&chunk).unwrap();

        let dir_path = format!(
            "{}/chunk_{}_{}",
            self.chunks_folder_path, chunk.pos.x, chunk.pos.y
        );

        std::fs::create_dir_all(&dir_path).expect("Failed to create chunk directory");

        let string_path = format!("{}/info.json", dir_path);
        let mut file = File::create(&string_path).unwrap();
        file.write_all(serialized_chunk.as_bytes()).unwrap();

        let noise_image = generate_noise_image(chunk_pos);
        let image_path = format!("{}/noise.png", dir_path);
        noise_image.save(image_path).unwrap();
    }

    fn load_chunk(&mut self, chunk_pos: LocalChunkPosition) {
        let dir_path = format!(
            "{}/chunk_{}_{}",
            self.chunks_folder_path, chunk_pos.x, chunk_pos.y
        );

        let string_path = format!("{}/info.json", dir_path);
        let file = File::open(&string_path).unwrap();
        let chunk: Chunk = serde_json::from_reader(file).unwrap();

        self.chunk_map.lock().unwrap().insert(chunk_pos, chunk);
    }

    fn spawn_chunk(&self, chunk: Chunk, commands: &mut Commands) {
        commands.spawn(chunk);
    }

    fn despawn_chunk(&self, chunk: Chunk, commands: &mut Commands, chunk_query: &Query<(Entity, &Chunk)>) {
        for (queried_chunk_entity, queried_chunk) in chunk_query.iter() {
            if chunk.pos == queried_chunk.pos {
                commands.entity(queried_chunk_entity).despawn();
                return;
            }
        }
    }

    fn unload_chunk(&mut self, chunk: Chunk) {
        self.chunk_map.lock().unwrap().remove(&chunk.pos);
    }
}
