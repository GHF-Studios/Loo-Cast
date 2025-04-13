use spacetime_engine_macros::define_workflow_mod;

define_workflow_mod!("Chunk", [
    workflow!("SpawnChunk")
    workflow!("DespawnChunk")
    workflow!("TransferChunkOwnership")
]);
