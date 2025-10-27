use bevy::ecs::schedule::ScheduleLabel;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpdateScaleMeter1;

// PreStartup
// Startup
// PostStartup
// 
// First
// PreUpdate
// Update
// PostUpdate
// Last
// 
// FixedFirst
// FixedPreUpdate
// FixedUpdate
// FixedPostUpdate
// FixedLast