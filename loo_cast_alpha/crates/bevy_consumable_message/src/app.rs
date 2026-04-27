use bevy_app::{App, First, SubApp};
use bevy_ecs::{message::Message, system::ResMut};

use crate::ConsumableMessages;

/// Extension trait for `bevy_app::App` to automatically add consumable messages.
pub trait ConsumableMessageApp {
    /// Adds a consumable message type of `M`.
    ///
    /// These messages are cleared at the start of each frame. Reads occuring
    /// *before* an message writer will **never** see the messages it produces.
    ///
    /// An example for when to use this is a click in a user interface. The click
    /// would be produced in PreUpdate, and once a system consumes the click, no
    /// other system can read that click. If a click gets to the end of the frame
    /// without being consumed, no UI elements need it, so the message should be
    /// discarded.
    fn add_consumable_message<M: Message>(&mut self) -> &mut Self;

    /// Adds a "persistent" consumable message type of `M`.
    ///
    /// Only consumed messages are cleared at the start of each frame. This allows
    /// users to consume messages whenever they want, even several frames after the
    /// message was triggered. However this can cause the messages to grow
    /// indefinitely if messages are not consumed.
    ///
    /// An example for when to use this is a line of customers. Customers can join
    /// the line at any time, but they can only be served at 1 customer per
    /// second. One system would write customer messages at a random rate, and
    /// another system would consume an message whenever a timer goes off.
    fn add_persistent_consumable_message<M: Message>(&mut self) -> &mut Self;
}

impl ConsumableMessageApp for App {
    fn add_consumable_message<M: Message>(&mut self) -> &mut Self {
        self.init_resource::<ConsumableMessages<M>>().add_systems(First, clear_all_messages::<M>)
    }

    fn add_persistent_consumable_message<M: Message>(&mut self) -> &mut Self {
        self.init_resource::<ConsumableMessages<M>>().add_systems(First, clear_consumed_messages::<M>)
    }
}

impl ConsumableMessageApp for SubApp {
    fn add_consumable_message<M: Message>(&mut self) -> &mut Self {
        self.init_resource::<ConsumableMessages<M>>().add_systems(First, clear_all_messages::<M>)
    }

    fn add_persistent_consumable_message<M: Message>(&mut self) -> &mut Self {
        self.init_resource::<ConsumableMessages<M>>().add_systems(First, clear_consumed_messages::<M>)
    }
}

/// A system for clearing all messages of type `M`.
fn clear_all_messages<M: Message>(mut messages: ResMut<ConsumableMessages<M>>) {
    messages.clear();
}

/// A system for clearing just the consumed messages of type `M`.
fn clear_consumed_messages<M: Message>(mut messages: ResMut<ConsumableMessages<M>>) {
    messages.clear_consumed();
}
