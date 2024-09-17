use bevy::prelude::*;

#[derive(Component)]
pub struct Serialized;

#[derive(Component)]
pub struct ReactOnAdd<T: Component> {
    callback: Box<dyn Fn(&T) + Send + Sync + 'static>,
}

impl<T: Component> ReactOnAdd<T> {
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        Self {
            callback: Box::new(callback),
        }
    }

    pub fn call(&self, component: &T) {
        (self.callback)(component);
    }
}

#[derive(Component)]
pub struct ReactOnRemove<T: Component> {
    callback: Box<dyn Fn(&T) + Send + Sync + 'static>,
}

impl<T: Component> ReactOnRemove<T> {
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        Self {
            callback: Box::new(callback),
        }
    }

    pub fn call(&self, component: &T) {
        (self.callback)(component);
    }
}