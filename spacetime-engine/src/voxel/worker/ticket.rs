//! Result publication and cancellation independent of compute admission.

use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
    mpsc::{Receiver, TryRecvError},
};

pub(in crate::voxel) struct VoxelWorkTicket<T: Send + 'static> {
    receiver: Mutex<Receiver<Result<T, VoxelWorkFailure>>>,
    cancelled: Arc<AtomicBool>,
}

#[derive(Debug, Clone, Copy)]
pub(in crate::voxel) enum VoxelWorkFailure {
    Panicked,
    Disconnected,
}

impl<T: Send + 'static> VoxelWorkTicket<T> {
    pub(super) fn new(
        receiver: Receiver<Result<T, VoxelWorkFailure>>,
        cancelled: Arc<AtomicBool>,
    ) -> Self {
        Self {
            receiver: Mutex::new(receiver),
            cancelled,
        }
    }

    /// Pending, ready and terminal failure are distinct outcomes. A lost
    /// worker must never leave an ECS task waiting forever.
    pub(in crate::voxel) fn try_take(&mut self) -> Result<Option<T>, VoxelWorkFailure> {
        let receiver = self
            .receiver
            .get_mut()
            .expect("voxel worker result mutex poisoned");
        match receiver.try_recv() {
            Ok(Ok(value)) => Ok(Some(value)),
            Ok(Err(failure)) => Err(failure),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(VoxelWorkFailure::Disconnected),
        }
    }
}

impl<T: Send + 'static> Drop for VoxelWorkTicket<T> {
    fn drop(&mut self) {
        // Result lifetime and compute-admission lifetime are independent.
        // Dropping the ticket prevents publication of stale output, while the
        // queued/running worker closure owns releasing compute admission.
        self.cancelled.store(true, Ordering::Release);
    }
}
