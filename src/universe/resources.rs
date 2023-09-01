use super::LocalChunkPosition;

use crate::noise::*;

use bevy::prelude::*;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use image::*;

#[derive(Resource)]
pub struct UniverseManager {
    pub current_scale_level: i8,
    pub current_chunk_offset_x: i16,
    pub current_chunk_offset_y: i16,
}

impl UniverseManager {
    pub fn new() -> Self {
        Self {
            current_scale_level: 0,
            current_chunk_offset_x: 0,
            current_chunk_offset_y: 0,
        }
    }
}

struct ChunkRequest {
    position: LocalChunkPosition,
    callback: Box<dyn Fn(GrayImage) + Send>,
}

#[derive(Resource)]
pub struct ChunkGenerator {
    tx_to_worker: Arc<Mutex<mpsc::Sender<ChunkRequest>>>,
}

impl ChunkGenerator {
    pub fn new() -> Self {
        let (tx_to_worker, rx_from_main) = mpsc::channel::<ChunkRequest>();
        
        let tx_to_worker = Arc::new(Mutex::new(tx_to_worker));
        let rx_from_main = Arc::new(Mutex::new(rx_from_main));
        
        thread::spawn(move || loop {
            let rx_lock = rx_from_main.lock().unwrap();
            
            if let Ok(request) = rx_lock.recv() {
                drop(rx_lock); // Explicitly drop the lock
                
                let noise = generate_noise_image(request.position);
                
                (request.callback)(noise);
            }
        });

        Self {
            tx_to_worker,
        }
    }

    pub fn request_chunk(&self, position: LocalChunkPosition, callback: Box<dyn Fn(GrayImage) + Send>) {
        let tx_lock = self.tx_to_worker.lock().unwrap();
        tx_lock.send(ChunkRequest {
            position,
            callback,
        }).unwrap();
    }
}
