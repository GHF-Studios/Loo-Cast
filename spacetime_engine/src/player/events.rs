use bevy::prelude::*;
use super::structs::*;

// prom0te and dem0te and prm0ted and dem0ted are not yet found and replaced

// API Operations
/// API Request to promote an entity to a player
/// 
#[derive(Debug, Clone, Event)]
pub struct PromoteToPlayer(pub PlayerRequest);

#[derive(Debug, Clone, Event)]
pub struct DemoteFromPlayer(pub PlayerRequest);

#[derive(Debug, Clone, Event)]
pub(super) struct PromoteToPlayerInternal(pub InternalPlayerRequest);

#[derive(Debug, Clone, Event)]
pub(super) struct DemoteFromPlayerInternal(pub InternalPlayerRequest);

// API Responses
#[derive(Debug, Clone, Event)]
pub(crate) struct PromotedToPlayerInternal(pub InternalPlayerResponse);

#[derive(Debug, Clone, Event)]
pub(crate) struct DemotedFromPlayerInternal(pub InternalPlayerResponse);

#[derive(Debug, Clone, Event)]
pub struct PromotedToPlayer(pub PlayerResponse);

#[derive(Debug, Clone, Event)]
pub struct DemotedFromPlayer(pub PlayerResponse);

// Lifecycle Operations
#[derive(Debug, Clone, Event)]
pub struct StartedPlayer(pub PlayerResponse);

#[derive(Debug, Clone, Event)]
pub struct StoppedPlayer(pub PlayerResponse);
