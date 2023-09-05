use super::LocalChunkPosition;
use super::components::Chunk;

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
    InProgressForGeneration,
    Generated,
    QueuedForLoading,
    InProgressForLoading,
    Loaded,
    QueuedForSpawning,
    InProgressForSpawning,
    Spawned,
    QueuedForDespawning,
    InProgressForDespawning,
    Despawned,
    QueuedForUnloading,
    InProgressForUnloading,
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

// Update the processing loop, so it actually updates the state of the processed chunk, and also minimize the time that a lock is held
// Make the callbacks be concurrent
// Make the callbacks actually be cancellable
// Update enqueue_X methods to follow new enqueue rules

// New Enqueue Rules:
// For an unload to be requested, the chunk has to either be: requested for load, loading, loaded, or despawned.
// For a despawn to be requested, the chunk has to be: requested for spawn, spawning, or spawned
// For a spawn to be requested, the chunk has to either be: requested for despawn, despawning, despawned, or loaded
// For a load to be requested, the chunk has to either be: requested for unload, unloading, unloaded, or generated
// For a generate to be requested, the chunk has to be: unregistered
// Unload opposes Load
// Despawn opposes Spawn
// When enqueueing an operation: If the requested operation opposes an enqueued operation which is not yet being processed, then the enqueued operation is cancelled and the requested operation is ignored
// When enqueueing an operation: If the requested operation opposes an enqueued operation which is being processed, then the enqueued operation's callback is cancelled and the requested operation is enqueued
// WHen enqueueing an operation: If the requested operation conflicts with the current state of the chunk, then the operation request returns an Error, detailing the state conflict

#[derive(Resource)]
pub struct ChunkManager {
    chunks_folder_path: String,
    chunk_info_map: HashMap<LocalChunkPosition, ChunkInfo>,
    chunk_map: HashMap<LocalChunkPosition, Chunk>,

    generate_queue: Arc<Mutex<VecDeque<LocalChunkPosition>>>,
    load_queue: Arc<Mutex<VecDeque<LocalChunkPosition>>>,
    spawn_queue: Arc<Mutex<VecDeque<LocalChunkPosition>>>,
    despawn_queue: Arc<Mutex<VecDeque<LocalChunkPosition>>>,
    unload_queue: Arc<Mutex<VecDeque<LocalChunkPosition>>>,

    generate_callbacks: HashMap<LocalChunkPosition, CallbackInfo>,
    load_callbacks: HashMap<LocalChunkPosition, CallbackInfo>,
    spawn_callbacks: HashMap<LocalChunkPosition, CallbackInfo>,
    despawn_callbacks: HashMap<LocalChunkPosition, CallbackInfo>,
    unload_callbacks: HashMap<LocalChunkPosition, CallbackInfo>,

    is_processing_generate_operations: Arc<Mutex<bool>>,
    is_processing_load_operations: Arc<Mutex<bool>>,
    is_processing_spawn_operations: Arc<Mutex<bool>>,
    is_processing_despawn_operations: Arc<Mutex<bool>>,
    is_processing_unload_operations: Arc<Mutex<bool>>,

    tx_enqueue_generate: Arc<Mutex<mpsc::Sender<LocalChunkPosition>>>,
    tx_enqueue_load: Arc<Mutex<mpsc::Sender<LocalChunkPosition>>>,
    tx_enqueue_spawn: Arc<Mutex<mpsc::Sender<LocalChunkPosition>>>,
    tx_enqueue_despawn: Arc<Mutex<mpsc::Sender<LocalChunkPosition>>>,
    tx_enqueue_unload: Arc<Mutex<mpsc::Sender<LocalChunkPosition>>>,

    tx_process_generate: Arc<Mutex<mpsc::Sender<usize>>>,
    tx_process_load: Arc<Mutex<mpsc::Sender<usize>>>,
    tx_process_spawn: Arc<Mutex<mpsc::Sender<usize>>>,
    tx_process_despawn: Arc<Mutex<mpsc::Sender<usize>>>,
    tx_process_unload: Arc<Mutex<mpsc::Sender<usize>>>,
}

struct ChunkManagerInitializationData {
    rx_enqueue_generate: mpsc::Receiver<LocalChunkPosition>,
    rx_enqueue_load: mpsc::Receiver<LocalChunkPosition>,
    rx_enqueue_spawn: mpsc::Receiver<LocalChunkPosition>,
    rx_enqueue_despawn: mpsc::Receiver<LocalChunkPosition>,
    rx_enqueue_unload: mpsc::Receiver<LocalChunkPosition>,

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

    pub fn is_processing_generate_operations(&self) -> bool {
        *self.is_processing_generate_operations.lock().unwrap()
    }

    pub fn is_processing_load_operations(&self) -> bool {
        *self.is_processing_load_operations.lock().unwrap()
    }

    pub fn is_processing_spawn_operations(&self) -> bool {
        *self.is_processing_spawn_operations.lock().unwrap()
    }

    pub fn is_processing_despawn_operations(&self) -> bool {
        *self.is_processing_despawn_operations.lock().unwrap()
    }

    pub fn is_processing_unload_operations(&self) -> bool {
        *self.is_processing_unload_operations.lock().unwrap()
    }

    pub fn enqueue_generate_operation(&self, position: LocalChunkPosition, callback: Callback) {
        self.tx_enqueue_generate
            .lock()
            .unwrap()
            .send(position)
            .unwrap();
    
        self.generate_callbacks.insert(position, CallbackInfo {
            callback,
            cancelled: false,
        });
    }

    pub fn enqueue_load_operation(&self, position: LocalChunkPosition, callback: Callback) {
        self.tx_enqueue_load
            .lock()
            .unwrap()
            .send(position)
            .unwrap();
    
        self.load_callbacks.insert(position, CallbackInfo {
            callback,
            cancelled: false,
        });
    }

    pub fn enqueue_spawn_operation(&self, position: LocalChunkPosition, callback: Callback) {
        self.tx_enqueue_spawn
            .lock()
            .unwrap()
            .send(position)
            .unwrap();
    
        self.spawn_callbacks.insert(position, CallbackInfo {
            callback,
            cancelled: false,
        });
    }

    pub fn enqueue_despawn_operation(&self, position: LocalChunkPosition, callback: Callback) {
        self.tx_enqueue_despawn
            .lock()
            .unwrap()
            .send(position)
            .unwrap();
    
        self.despawn_callbacks.insert(position, CallbackInfo {
            callback,
            cancelled: false,
        });
    }

    pub fn enqueue_unload_operation(&self, position: LocalChunkPosition, callback: Callback) {
        self.tx_enqueue_unload
            .lock()
            .unwrap()
            .send(position)
            .unwrap();
    
        self.unload_callbacks.insert(position, CallbackInfo {
            callback,
            cancelled: false,
        });
    }

    pub fn process_generate_operations(&self, batch_size: usize) {
        self.tx_process_generate
            .lock()
            .unwrap()
            .send(batch_size)
            .unwrap();
    }

    pub fn process_load_operations(&self, batch_size: usize) {
        self.tx_process_load
            .lock()
            .unwrap()
            .send(batch_size)
            .unwrap();
    }

    pub fn process_spawn_operations(&self, batch_size: usize) {
        self.tx_process_spawn
            .lock()
            .unwrap()
            .send(batch_size)
            .unwrap();
    }

    pub fn process_despawn_operations(&self, batch_size: usize) {
        self.tx_process_despawn
            .lock()
            .unwrap()
            .send(batch_size)
            .unwrap();
    }

    pub fn process_unload_operations(&self, batch_size: usize) {
        self.tx_process_unload
            .lock()
            .unwrap()
            .send(batch_size)
            .unwrap();
    }

    pub fn get_chunk_state(&self, position: LocalChunkPosition) -> Option<ChunkState> {
        if let Some(chunk_info) = self.chunk_info_map.get(&position) {
            Some(chunk_info.state.clone())
        } else {
            None
        }
    }

    fn create_instance(chunks_folder_path: String) -> (Self, ChunkManagerInitializationData) {
        let (tx_enqueue_generate, rx_enqueue_generate) = mpsc::channel::<LocalChunkPosition>();
        let (tx_enqueue_load, rx_enqueue_load) = mpsc::channel::<LocalChunkPosition>();
        let (tx_enqueue_spawn, rx_enqueue_spawn) = mpsc::channel::<LocalChunkPosition>();
        let (tx_enqueue_despawn, rx_enqueue_despawn) = mpsc::channel::<LocalChunkPosition>();
        let (tx_enqueue_unload, rx_enqueue_unload) = mpsc::channel::<LocalChunkPosition>();

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
                chunk_info_map: HashMap::new(),
                chunk_map: HashMap::new(),

                generate_queue,
                load_queue,
                spawn_queue,
                despawn_queue,
                unload_queue,

                generate_callbacks: HashMap::new(),
                load_callbacks: HashMap::new(),
                spawn_callbacks: HashMap::new(),
                despawn_callbacks: HashMap::new(),
                unload_callbacks: HashMap::new(),

                is_processing_generate_operations: Arc::new(Mutex::new(false)),
                is_processing_load_operations: Arc::new(Mutex::new(false)),
                is_processing_spawn_operations: Arc::new(Mutex::new(false)),
                is_processing_despawn_operations: Arc::new(Mutex::new(false)),
                is_processing_unload_operations: Arc::new(Mutex::new(false)),

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
        let generate_queue_lock = self.generate_queue.clone();
        let load_queue_lock = self.load_queue.clone();
        let spawn_queue_lock = self.spawn_queue.clone();
        let despawn_queue_lock = self.despawn_queue.clone();
        let unload_queue_lock = self.unload_queue.clone();

        let is_processing_generate_operations = self.is_processing_generate_operations.clone();
        let is_processing_load_operations = self.is_processing_load_operations.clone();
        let is_processing_spawn_operations = self.is_processing_spawn_operations.clone();
        let is_processing_despawn_operations = self.is_processing_despawn_operations.clone();
        let is_processing_unload_operations = self.is_processing_unload_operations.clone();

        thread::spawn(move || loop {
            while let Ok(requested_position) = initialization_data.rx_enqueue_generate.recv() {
                let mut queue = generate_queue_lock.lock().unwrap();
                queue.push_back(requested_position);
            }
            while let Ok(requested_position) = initialization_data.rx_enqueue_load.recv() {
                let mut queue = load_queue_lock.lock().unwrap();
                queue.push_back(requested_position);
            }
            while let Ok(requested_position) = initialization_data.rx_enqueue_spawn.recv() {
                let mut queue = spawn_queue_lock.lock().unwrap();
                queue.push_back(requested_position);
            }
            while let Ok(requested_position) = initialization_data.rx_enqueue_despawn.recv() {
                let mut queue = despawn_queue_lock.lock().unwrap();
                queue.push_back(requested_position);
            }
            while let Ok(requested_position) = initialization_data.rx_enqueue_unload.recv() {
                let mut queue = unload_queue_lock.lock().unwrap();
                queue.push_back(requested_position);
            }

            if let Ok(requested_batch_size) = initialization_data.rx_process_generate.recv() {
                *is_processing_generate_operations.lock().unwrap() = true;
                println!("Started processing generate operations");

                let mut queue = generate_queue_lock.lock().unwrap();
                for _ in 0..requested_batch_size {
                    if let Some(requested_position) = queue.pop_front() {
                        println!("Generating chunk at position {:?}", requested_position);
                    
                        if let Some(callback_info) = self.generate_callbacks.get(&requested_position) {
                            if !callback_info.cancelled {
                                (callback_info.callback)();
                            }
                        }
                    }
                }

                println!("Finished processing generate operations");
                *is_processing_generate_operations.lock().unwrap() = false;
            }
            if let Ok(requested_batch_size) = initialization_data.rx_process_load.recv() {
                *is_processing_load_operations.lock().unwrap() = true;
                println!("Started processing load operations");

                let mut queue = load_queue_lock.lock().unwrap();
                for _ in 0..requested_batch_size {
                    if let Some(requested_position) = queue.pop_front() {
                        println!("Loading chunk at position {:?}", requested_position);

                        if let Some(callback_info) = self.load_callbacks.get(&requested_position) {
                            if !callback_info.cancelled {
                                (callback_info.callback)();
                            }
                        }
                    }
                }

                println!("Finished processing load operations");
                *is_processing_load_operations.lock().unwrap() = false;
            }
            if let Ok(requested_batch_size) = initialization_data.rx_process_spawn.recv() {
                *is_processing_spawn_operations.lock().unwrap() = true;
                println!("Started processing spawn operations");

                let mut queue = spawn_queue_lock.lock().unwrap();
                for _ in 0..requested_batch_size {
                    if let Some(requested_position) = queue.pop_front() {
                        println!("Spawning chunk at position {:?}", requested_position);

                        if let Some(callback_info) = self.spawn_callbacks.get(&requested_position) {
                            if !callback_info.cancelled {
                                (callback_info.callback)();
                            }
                        }
                    }
                }

                println!("Finished processing spawn operations");
                *is_processing_spawn_operations.lock().unwrap() = false;
            }
            if let Ok(requested_batch_size) = initialization_data.rx_process_despawn.recv() {
                *is_processing_despawn_operations.lock().unwrap() = true;
                println!("Started processing despawn operations");

                let mut queue = despawn_queue_lock.lock().unwrap();
                for _ in 0..requested_batch_size {
                    if let Some(requested_position) = queue.pop_front() {
                        println!("Despawning chunk at position {:?}", requested_position);

                        if let Some(callback_info) = self.despawn_callbacks.get(&requested_position) {
                            if !callback_info.cancelled {
                                (callback_info.callback)();
                            }
                        }
                    }
                }

                println!("Finished processing despawn operations");
                *is_processing_despawn_operations.lock().unwrap() = false;
            }
            if let Ok(requested_batch_size) = initialization_data.rx_process_unload.recv() {
                *is_processing_unload_operations.lock().unwrap() = true;
                println!("Started processing unload operations");

                let mut queue = unload_queue_lock.lock().unwrap();
                for _ in 0..requested_batch_size {
                    if let Some(requested_position) = queue.pop_front() {
                        println!("Unloading chunk at position {:?}", requested_position);

                        if let Some(callback_info) = self.unload_callbacks.get(&requested_position) {
                            if !callback_info.cancelled {
                                (callback_info.callback)();
                            }
                        }
                    }
                }

                println!("Finished processing unload operations");
                *is_processing_unload_operations.lock().unwrap() = false;
            }
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

        self.chunk_map.insert(chunk_pos, chunk);
    }

    fn spawn_chunk(&self, chunk: Chunk, commands: &mut Commands,) {
        commands.spawn(chunk);
    }

    fn despawn_chunk(&self, chunk: Chunk, commands: &mut Commands, chunk_query: &Query<(Entity, &Chunk)>,) {
        for (queried_chunk_entity, queried_chunk) in chunk_query.iter() {
            if chunk.pos == queried_chunk.pos {
                commands.entity(queried_chunk_entity).despawn();
                return;
            }
        }
    }

    fn unload_chunk(&mut self, chunk: Chunk,) {
        self.chunk_map.remove(&chunk.pos);
    }
}
