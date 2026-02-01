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
pub struct ConsumableMessages<E: Message> {
    /// The messages in the buffer. `None` implies that the message there was
    /// consumed. `Some` means that the message has not been consumed yet.
    messages: Vec<Option<E>>,
}

// Derived Default impl would incorrectly require E: Default
impl<E: Message> Default for ConsumableMessages<E> {
    fn default() -> Self {
        Self { messages: Default::default() }
    }
}

impl<E: Message> ConsumableMessages<E> {
    /// "Sends" `message` by writing it to the buffer. [`read`] can then read the
    /// message.
    pub fn send(&mut self, message: E) {
        self.messages.push(Some(message));
    }

    /// Sends a list of `messages` all at once, which can later be [`read`]. This is
    /// more efficient than sending each message individually.
    pub fn send_batch(&mut self, messages: impl IntoIterator<Item = E>) {
        self.extend(messages);
    }

    /// Sends the default value of the message. Useful when the message is an empty
    /// struct.
    pub fn send_default(&mut self)
    where
        E: Default,
    {
        self.send(Default::default())
    }

    /// Reads the unconsumed messages stored in self.
    pub fn read(&mut self) -> ConsumableMessageIterator<'_, E> {
        ConsumableMessageIterator { iter: self.messages.iter_mut() }
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

impl<E: Message> Extend<E> for ConsumableMessages<E> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = E>,
    {
        self.messages.extend(iter.into_iter().map(|message| Some(message)));
    }
}

/// Mutable borrow of a consumable message.
pub struct Consume<'messages, E> {
    /// The message itself.
    message: &'messages mut Option<E>,
}

impl<'messages, E> Deref for Consume<'messages, E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        self.message.as_ref().expect("The message has not been consumed yet.")
    }
}

impl<'messages, E> DerefMut for Consume<'messages, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.message.as_mut().expect("The message has not been consumed yet.")
    }
}

impl<'messages, E> Consume<'messages, E> {
    /// Consumes the message.
    pub fn consume(self) -> E {
        self.message.take().expect("The message has not been consumed until now.")
    }
}

/// Sends consumable messages of type `E`.
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
pub struct ConsumableMessageWriter<'w, E: Message> {
    /// The messages to write to.
    messages: ResMut<'w, ConsumableMessages<E>>,
}

impl<'w, E: Message> ConsumableMessageWriter<'w, E> {
    /// "Sends" `message` by writing it to the buffer. [`ConsumableMessageReader`] can
    /// then read the message.
    pub fn send(&mut self, message: E) {
        self.messages.send(message);
    }

    /// Sends a list of `messages` all at once, which can later be [`read`]. This is
    /// more efficient than sending each message individually.
    pub fn send_batch(&mut self, messages: impl IntoIterator<Item = E>) {
        self.messages.send_batch(messages);
    }

    /// Sends the default value of the message. Useful when the message is an empty
    /// struct.
    pub fn send_default(&mut self)
    where
        E: Default,
    {
        self.messages.send_default()
    }
}

/// Reads (and possibly consumes) messages of type `E`.
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
pub struct ConsumableMessageReader<'w, E: Message> {
    /// The messages to read from.
    messages: ResMut<'w, ConsumableMessages<E>>,
}

impl<'w, E: Message> ConsumableMessageReader<'w, E> {
    /// Reads the unconsumed messages.
    pub fn read(&mut self) -> ConsumableMessageIterator<'_, E> {
        self.messages.read()
    }

    /// Reads all unconsumed messages, consuming them all along the way.
    pub fn read_and_consume_all(&mut self) -> impl Iterator<Item = E> + '_ {
        // TODO: The lifetime bounds of this function are wrong. Rust 2024 edition
        // fixes this, but for now, this will most likely be fine.
        self.messages.read().map(|message| message.consume())
    }
}

/// An iterator over the unconsumed messages.
#[derive(Debug)]
pub struct ConsumableMessageIterator<'w, E: Message> {
    /// The iterator being wrapped.
    iter: IterMut<'w, Option<E>>,
}

impl<'w, E: Message> Iterator for ConsumableMessageIterator<'w, E> {
    type Item = Consume<'w, E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|message| message.is_some()).map(|message| Consume { message })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}
