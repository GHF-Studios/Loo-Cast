// Modules

// Local imports

// Internal imports

// External imports

// Static variables

// Constant variables

// Types

// Traits
pub trait Manager {
    fn initialize(&mut self) -> Result<(), ManagerInitializeError>;
    fn finalize(&mut self) -> Result<(), ManagerFinalizeError>;
    fn get_state(&self) -> &ManagerState;
}

// Enums
pub enum ManagerState {
    Created,
    Initialized,
    Finalized,
}

#[derive(Debug)]
pub enum ManagerInitializeError {
    ManagerAlreadyInitialized,
    ManagerAlreadyFinalized,
}

#[derive(Debug)]
pub enum ManagerFinalizeError {
    ManagerNotInitialized,
    ManagerAlreadyFinalized,
}

// Structs

// Implementations

// Module Functions
