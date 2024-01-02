// Modules

// Local imports

// Internal imports

// External imports
use std::{any::*, collections::HashMap};
use std::sync::{Arc, Mutex};
use lazy_static::*;

// Static variables
lazy_static! {
    pub static ref EVENT_MANAGER: Arc<Mutex<EventManager>> = Arc::new(Mutex::new(EventManager::new()));
}

// Constant variables

// Types


// Traits
pub trait Event: Any + Clone {
}

// Enums

// Structs
pub struct EventManager {
    event_types: Vec<TypeId>,
    event_subscribers: HashMap<TypeId, HashMap<usize, Box<dyn Fn(Box<dyn Any>) + Send>>>,
    next_subscriber_id: usize,
}

pub struct EventSubscriberHandle {
    subscriber_id: usize,
    event_type: TypeId,
    event_name: String,
}

// Implementations
impl EventManager {
    pub fn new() -> EventManager {
        EventManager {
            event_types: Vec::new(),
            event_subscribers: HashMap::new(),
            next_subscriber_id: 0,
        }
    }

    pub fn register_event_type<T: Event>(&mut self) -> Result<(), String> {
        let event_type = TypeId::of::<T>();
        let event_name = type_name::<T>();

        if self.event_types.contains(&event_type) {
            return Err(format!("Event type {} already registered", event_name));
        }

        self.event_types.push(event_type);

        Ok(())
    }

    pub fn unregister_event_type<T: Event>(&mut self) -> Result<(), String> {
        let event_type = TypeId::of::<T>();
        let event_name = type_name::<T>();

        if !self.event_types.contains(&event_type) {
            return Err(format!("Event type {} not registered", event_name));
        }

        self.event_types.retain(|&x| x != event_type);

        Ok(())
    }

    pub fn subscribe<T: Event>(&mut self, event_handler: Box<dyn Fn(T) + Send>) -> Result<EventSubscriberHandle, String> {
        let event_type = TypeId::of::<T>();
        let event_name = type_name::<T>();

        if !self.event_types.contains(&event_type) {
            return Err(format!("Event type {} not registered", event_name));
        }

        let subscriber_id = self.next_subscriber_id;
        self.next_subscriber_id += 1;

        let event_subscribers = self.event_subscribers.entry(event_type).or_insert(HashMap::new());

        if event_subscribers.contains_key(&subscriber_id) {
            return Err(format!("Subscriber {} already registered for event type {}", subscriber_id, event_name));
        }

        event_subscribers.insert(subscriber_id, Box::new(move |event| {
            let event = event.downcast::<T>().unwrap();
            event_handler(*event);
        }));

        Ok(EventSubscriberHandle {
            subscriber_id,
            event_type,
            event_name: event_name.to_string(),
        })
    }

    pub fn unsubscribe(&mut self, subscriber_handle: EventSubscriberHandle) -> Result<(), String> {
        let event_type = subscriber_handle.event_type;
        let event_name = subscriber_handle.event_name;

        if !self.event_types.contains(&event_type) {
            return Err(format!("Event type {} not registered", event_name));
        }

        let event_subscribers = self.event_subscribers.entry(event_type).or_insert(HashMap::new());

        if let None = event_subscribers.remove(&subscriber_handle.subscriber_id) {
            return Err(format!("Subscriber {} not registered", subscriber_handle.subscriber_id));
        }

        Ok(())
    }

    pub fn publish<T: Event>(&mut self, event: T) -> Result<(), String> {
        let event_type = TypeId::of::<T>();
        let event_name = type_name::<T>();

        if !self.event_types.contains(&event_type) {
            return Err(format!("Event type {} not registered", event_name));
        }

        let event_subscribers = self.event_subscribers.entry(event_type).or_insert(HashMap::new());

        for (_, event_handler) in event_subscribers {
            event_handler(Box::new(event.clone()));
        }

        Ok(())
    }
}

// Module Functions
