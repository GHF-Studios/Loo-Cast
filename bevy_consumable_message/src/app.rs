use bevy_app::{App, First, SubApp};
use bevy_ecs::{message::Message, system::ResMut};

use crate::ConsumableMessages;

/// Extension trait for `bevy_app::App` to automatically add consumable messages.
pub trait ConsumableMessageApp {
    /// Adds a consumable message type of `E`.
    ///
    /// These messages are cleared at the start of each frame. Reads occuring
    /// *before* an message writer will **never** see the messages it produces.
    ///
    /// An example for when to use this is a click in a user interface. The click
    /// would be produced in PreUpdate, and once a system consumes the click, no
    /// other system can read that click. If a click gets to the end of the frame
    /// without being consumed, no UI elements need it, so the message should be
    /// discarded.
    fn add_consumable_message<E: Message>(&mut self) -> &mut Self;

    /// Adds a "persistent" consumable message type of `E`.
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
    fn add_persistent_consumable_message<E: Message>(&mut self) -> &mut Self;
}

impl ConsumableMessageApp for App {
    fn add_consumable_message<E: Message>(&mut self) -> &mut Self {
        self.init_resource::<ConsumableMessages<E>>().add_systems(First, clear_all_messages::<E>)
    }

    fn add_persistent_consumable_message<E: Message>(&mut self) -> &mut Self {
        self.init_resource::<ConsumableMessages<E>>().add_systems(First, clear_consumed_messages::<E>)
    }
}

impl ConsumableMessageApp for SubApp {
    fn add_consumable_message<E: Message>(&mut self) -> &mut Self {
        self.init_resource::<ConsumableMessages<E>>().add_systems(First, clear_all_messages::<E>)
    }

    fn add_persistent_consumable_message<E: Message>(&mut self) -> &mut Self {
        self.init_resource::<ConsumableMessages<E>>().add_systems(First, clear_consumed_messages::<E>)
    }
}

/// A system for clearing all messages of type `E`.
fn clear_all_messages<E: Message>(mut messages: ResMut<ConsumableMessages<E>>) {
    messages.clear();
}

/// A system for clearing just the consumed messages of type `E`.
fn clear_consumed_messages<E: Message>(mut messages: ResMut<ConsumableMessages<E>>) {
    messages.clear_consumed();
}
