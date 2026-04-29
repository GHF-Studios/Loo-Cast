//! bevy_consumable_message
//!
//! Lightweight utility crate that provides a consumable message collection and convenient
//! SystemParams for Bevy ECS. Messages can be read multiple times but consumed only once — a
//! small pattern useful for ordered, single-consumption events.
//!
//! The crate is compact and intended for internal reuse across workspace crates, but its types
//! are also generally useful as local utilities.

use std::{
    ops::{Deref, DerefMut},
    slice::IterMut,
};

use bevy_ecs::{
    message::Message,
    resource::Resource,
    system::{ResMut, SystemParam},
};

#[cfg(feature = "bevy_app")]
mod app;
#[cfg(feature = "bevy_app")]
pub use app::ConsumableMessageApp;

/// A collection of messages that can be consumed.
/// Messages can be written by using a [`ConsumableMessageWriter`] and are typically
/// read using a [`ConsumableMessageReader`].
///
/// Messages can be read many times, but only "consumed" once. See
/// [`ConsumableMessageReader`] for details.
///
/// Generally, all systems using `ConsumableMessages` should be stricly
/// [ordered](https://github.com/bevyengine/bevy/blob/main/examples/ecs/ecs_guide.rs).
/// This ensures that messages are consumed in a consistent order.
///
/// Messages will remain readable in this collection until they are consumed or
/// [`ConsumableMessages::clear`] is called. Messages that have been consumed will
/// only be fully removed once [`ConsumableMessages::clear_consumed`] or
/// [`ConsumableMessages::clear`] is called.
///
/// Typically, `ConsumableMessages` are initialized automatically using the
/// [`ConsumableMessageApp`] extension trait.
///
/// # Example
/// ```rust
/// use bevy_ecs::message::Message;
/// use bevy_consumable_message::ConsumableMessages;
///
/// #[derive(Message)]
/// struct MyMessage {
///     value: usize,
/// }
///
/// // setup
/// let mut messages = ConsumableMessages::<MyMessage>::default();
///
/// // somewhere else: send an message
/// messages.send(MyMessage { value: 1 });
///
/// // somewhere else: read the messages, and even mutate them.
/// for mut message in messages.read() {
///   assert_eq!(message.value, 1);
///   message.value = 2;
/// }
///
/// // somewhere else: consume the message
/// for mut message in messages.read() {
///   assert_eq!(message.value, 2);
///   message.consume();
/// }
///
/// // somewhere else: read the messages (nothing left).
/// assert_eq!(messages.read().count(), 0);
/// ```
#[derive(Resource)]
pub struct ConsumableMessages<M: Message> {
    /// The messages in the buffer. `None` implies that the message there was
    /// consumed. `Some` means that the message has not been consumed yet.
    messages: Vec<Option<M>>,
}

// Derived Default impl would incorrectly require M: Default
impl<M: Message> Default for ConsumableMessages<M> {
    fn default() -> Self {
        Self { messages: Default::default() }
    }
}

impl<M: Message> ConsumableMessages<M> {
    /// "Sends" `message` by writing it to the buffer. [`read`] can then read the
    /// message.
    pub fn send(&mut self, message: M) {
        self.messages.push(Some(message));
    }

    /// Sends a list of `messages` all at once, which can later be [`read`]. This is
    /// more efficient than sending each message individually.
    pub fn send_batch(&mut self, messages: impl IntoIterator<Item=M>) {
        self.extend(messages);
    }

    /// Sends the default value of the message. Useful when the message is an empty
    /// struct.
    pub fn send_default(&mut self)
    where
        M: Default,
    {
        self.send(Default::default())
    }

    /// Reads the unconsumed messages stored in self.
    pub fn read(&mut self) -> ConsumableMessageIterator<'_, M> {
        ConsumableMessageIterator {
            iter: self.messages.iter_mut(),
        }
    }

    /// Clears all messages stored in self. Unconsumed messages are also dropped.
    pub fn clear(&mut self) {
        self.messages.clear();
    }

    /// Clears only consumed messages stored in self. This is not strictly required,
    /// but calling it regularly will reduce memory usage (since the consumed
    /// messages cannot be read anyway).
    pub fn clear_consumed(&mut self) {
        self.messages.retain(|message| message.is_some());
    }
}

impl<M: Message> Extend<M> for ConsumableMessages<M> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item=M>,
    {
        self.messages.extend(iter.into_iter().map(|message| Some(message)));
    }
}

/// Mutable borrow of a consumable message.
pub struct Consume<'messages, M> {
    /// The message itself.
    message: &'messages mut Option<M>,
}

impl<'messages, M> Deref for Consume<'messages, M> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        self.message.as_ref().expect("The message has not been consumed yet.")
    }
}

impl<'messages, M> DerefMut for Consume<'messages, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.message.as_mut().expect("The message has not been consumed yet.")
    }
}

impl<'messages, M> Consume<'messages, M> {
    /// Consumes the message.
    pub fn consume(self) -> M {
        self.message.take().expect("The message has not been consumed until now.")
    }
}

/// Sends consumable messages of type `M`.
///
/// # Usage
///
/// `ConsumableMessageWriter`s are usually declared as a [`SystemParam`].
/// ```
/// use bevy_ecs::prelude::*;
/// use bevy_consumable_message::ConsumableMessageWriter;
///
/// #[derive(Message)]
/// pub struct MyMessage; // Custom message type.
///
/// fn my_system(mut writer: ConsumableMessageWriter<MyMessage>) {
///   writer.send(MyMessage);
/// }
///
/// bevy_ecs::system::assert_is_system(my_system);
/// ```
#[derive(SystemParam)]
pub struct ConsumableMessageWriter<'w, M: Message> {
    /// The messages to write to.
    messages: ResMut<'w, ConsumableMessages<M>>,
}

impl<'w, M: Message> ConsumableMessageWriter<'w, M> {
    /// "Sends" `message` by writing it to the buffer. [`ConsumableMessageReader`] can
    /// then read the message.
    pub fn send(&mut self, message: M) {
        self.messages.send(message);
    }

    /// Sends a list of `messages` all at once, which can later be [`read`]. This is
    /// more efficient than sending each message individually.
    pub fn send_batch(&mut self, messages: impl IntoIterator<Item=M>) {
        self.messages.send_batch(messages);
    }

    /// Sends the default value of the message. Useful when the message is an empty
    /// struct.
    pub fn send_default(&mut self)
    where
        M: Default,
    {
        self.messages.send_default()
    }
}

/// Reads (and possibly consumes) messages of type `M`.
///
/// # Usage
///
/// `ConsumableMessageReader`s are usually declared as a [`SystemParam`].
/// ```
/// use bevy_ecs::prelude::*;
/// use bevy_consumable_message::ConsumableMessageReader;
///
/// #[derive(Message, Debug)]
/// pub struct MyMessage; // Custom message type.
///
/// fn my_system(mut reader: ConsumableMessageReader<MyMessage>) {
///   for mut message in reader.read() {
///     println!("{:?}", *message);
///     message.consume();
///   }
/// }
///
/// bevy_ecs::system::assert_is_system(my_system);
/// ```
#[derive(SystemParam)]
pub struct ConsumableMessageReader<'w, M: Message> {
    /// The messages to read from.
    messages: ResMut<'w, ConsumableMessages<M>>,
}

impl<'w, M: Message> ConsumableMessageReader<'w, M> {
    /// Reads the unconsumed messages.
    pub fn read(&mut self) -> ConsumableMessageIterator<'_, M> {
        self.messages.read()
    }

    /// Reads all unconsumed messages, consuming them all along the way.
    pub fn read_and_consume_all(&mut self) -> impl Iterator<Item=M> + '_ {
        // TODO: The lifetime bounds of this function are wrong. Rust 2024 edition
        // fixes this, but for now, this will most likely be fine.
        self.messages.read().map(|message| message.consume())
    }
}

/// An iterator over the unconsumed messages.
#[derive(Debug)]
pub struct ConsumableMessageIterator<'w, M: Message> {
    /// The iterator being wrapped.
    iter: IterMut<'w, Option<M>>,
}

impl<'w, M: Message> Iterator for ConsumableMessageIterator<'w, M> {
    type Item = Consume<'w, M>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|message| message.is_some()).map(|message| Consume { message })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}
