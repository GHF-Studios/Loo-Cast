use bevy::prelude::*;
use super::structs::*;

#[derive(Debug, Clone, Event)]
pub struct UpgradeToPlayer(pub PlayerRequest);

#[derive(Debug, Clone, Event)]
pub struct DowngradeFromPlayer(pub PlayerRequest);

#[derive(Debug, Clone, Event)]
pub(super) struct UpgradeToPlayerInternal(pub InternalPlayerRequest);

#[derive(Debug, Clone, Event)]
pub(super) struct DowngradeFromPlayerInternal(pub InternalPlayerRequest);

#[derive(Debug, Clone, Event)]
pub(crate) struct UpgradedToPlayerInternal(pub InternalPlayerResponse);

#[derive(Debug, Clone, Event)]
pub(crate) struct DowngradedFromPlayerInternal(pub InternalPlayerResponse);

#[derive(Debug, Clone, Event)]
pub struct UpgradedToPlayer(pub PlayerResponse);

#[derive(Debug, Clone, Event)]
pub struct DowngradedFromPlayer(pub PlayerResponse);

#[derive(Debug, Clone, Event)]
pub struct StartedPlayer(pub PlayerResponse);

#[derive(Debug, Clone, Event)]
pub struct StoppedPlayer(pub PlayerResponse);
