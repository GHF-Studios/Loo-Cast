The problem with the current Game/Universe/Player modules is that they strongly depend on each other, discouraging extensibility and felxibility.
The goal of this "bisl_planung_so_digga.rs" is to provide detailed comments and rust pseudocode for a completely re-imagined set of Game/Universe/Player modules.
These re-imagined modules will focus on their respective responsibilities and nothing more.
State (like the universe being Loaded, NotLoaded, etc.) should ideally be represented by bevy states; this way we can define startup, update and shutdown systems 
for whatever event we like, without strongly coupling these 3 modules to themselves and other modules.


Basic concepts (in order of startup):

Game(GameInfo, GameConfig, GameState), 
Universe(GlobalUniverse, LocalUniverses), 
Player


Basic lifecycle example

(CreateGame)
SelectGame
	SetCurrentGameInfo
LoadSelectedGame
	LoadCurrentGameConfig
	LoadCurrentGameState
LoadSelectedUniverse


Bit o' rust code, innit mate



pub struct GlobalUniverseInfo {
	id: u64
}

pub struct LocalUniverseInfo {
	id: u64,
	player_info: PlayerInfo,
}

pub struct ChunkInfo {
	id: ChunkID,
}

pub struct ChunkMetadata {
	parent_chunk_info: ChunkInfo
	child_chunk_infos: Vec<ChunkInfo>
	child_entity_infos: Vec<EntityInfo>
}

pub struct ChunkData {
	parent_chunk_metadata: ChunkMetadata,
	child_chunk_metadatas: Vec<Chunk>
}

pub enum Chunk {
	ChunkInfoLoaded {
		chunk_info: ChunkInfo
	},
	ChunkMetadataLoaded {
		
	},
	ChunkDataLoaded {
		
	},
}

pub struct EntityInfo {
	id: EntityID
}

pub struct EntityMetadata {
	
}

pub struct EntityData {
	
}

pub enum Entity {
	EntityInfoLoaded {
		
	},
	EntityMetadataLoaded {
		
	},
	EntityDataLoaded {
		
	},
}

pub struct UniverseManager {
	
}

pub struct PlayerManager {
	
}