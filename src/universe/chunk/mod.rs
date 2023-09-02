use super::LocalChunkPosition;

use crate::noise::*;

use bevy::prelude::*;
use image::*;
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive(PartialEq, Clone)]
pub enum ChunkStatus {
    Queued,
    InProgress,
}

struct ChunkRequest {
    position: LocalChunkPosition,
}

#[derive(Resource)]
pub struct ChunkGenerator {
    tx_to_worker: Arc<Mutex<mpsc::Sender<ChunkRequest>>>,
    chunk_status: Arc<Mutex<HashMap<LocalChunkPosition, ChunkStatus>>>,
    callbacks: Arc<Mutex<HashMap<LocalChunkPosition, Box<dyn Fn(GrayImage) + Send>>>>,
}

impl ChunkGenerator {
    pub fn new() -> Self {
        let (tx_to_worker, rx_from_main) = mpsc::channel::<ChunkRequest>();
        let tx_to_worker = Arc::new(Mutex::new(tx_to_worker));
        let rx_from_main = Arc::new(Mutex::new(rx_from_main));

        let chunk_status = Arc::new(Mutex::new(HashMap::<LocalChunkPosition, ChunkStatus>::new()));
        let callbacks = Arc::new(Mutex::new(HashMap::<
            LocalChunkPosition,
            Box<dyn Fn(GrayImage) + Send>,
        >::new()));

        let worker_chunk_status = chunk_status.clone();
        let worker_callbacks = callbacks.clone();

        thread::spawn(move || loop {
            let rx_lock = rx_from_main.lock().unwrap();
            if let Ok(request) = rx_lock.recv() {
                let mut status_lock = worker_chunk_status.lock().unwrap();
                let mut callbacks_lock = worker_callbacks.lock().unwrap();

                if let Some(status) = status_lock.get_mut(&request.position) {
                    if *status == ChunkStatus::Queued {
                        *status = ChunkStatus::InProgress;
                    } else {
                        continue;
                    }
                }

                let noise = generate_noise_image(request.position);

                if let Some(callback) = callbacks_lock.get(&request.position) {
                    callback(noise);
                }

                status_lock.remove(&request.position);
                callbacks_lock.remove(&request.position);
            }
        });

        Self {
            tx_to_worker,
            chunk_status,
            callbacks,
        }
    }

    pub fn request_chunk(
        &self,
        position: LocalChunkPosition,
        callback: Box<dyn Fn(GrayImage) + Send>,
    ) {
        let mut status_lock = self.chunk_status.lock().unwrap();
        let mut callback_lock = self.callbacks.lock().unwrap();

        if !status_lock.contains_key(&position) {
            status_lock.insert(position, ChunkStatus::Queued);
            callback_lock.insert(position, callback);

            let tx_lock = self.tx_to_worker.lock().unwrap();
            tx_lock.send(ChunkRequest { position }).unwrap();
        }
    }

    pub fn try_cancel_chunk_request(&self, position: LocalChunkPosition) -> Result<(), String> {
        let mut status_lock = self.chunk_status.lock().unwrap();
        let mut callback_lock = self.callbacks.lock().unwrap();
        if let Some(status) = status_lock.get_mut(&position) {
            if *status == ChunkStatus::Queued {
                status_lock.remove(&position);
                callback_lock.remove(&position);
                return Ok(());
            } else {
                return Err("Chunk is already being processed".to_string());
            }
        } else {
            return Err("Chunk is not queued".to_string());
        }
    }

    pub fn is_chunk_queued(&self, position: LocalChunkPosition) -> bool {
        let status_lock = self.chunk_status.lock().unwrap();
        status_lock.get(&position).is_some()
    }

    pub fn get_chunk_status(&self, position: LocalChunkPosition) -> Result<ChunkStatus, String> {
        let status_lock = self.chunk_status.lock().unwrap();
        match status_lock.get(&position) {
            Some(status) => Ok(status.clone()),
            None => Err("Chunk is not queued".to_string()),
        }
    }
}
