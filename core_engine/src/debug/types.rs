use std::any::Any;

pub struct AnySendNamedBox {
    name: String,
    inner: Box<dyn Any + Send>,
}

impl AnySendNamedBox {
    pub fn new<T: Any + Send + 'static>(value: T, name: String) -> Self {
        bevy::prelude::debug!("AnySendNamedBox'ed '{}'", name);
        
        Self {
            name,
            inner: Box::new(value),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn into_inner<T: Any>(self) -> T {
        let value = *self.inner
            .downcast()
            .unwrap_or_else(|_| {
                unreachable!("Failed to downcast AnySendNamedBox from '{}' to '{}'",
                    self.name,
                    std::any::type_name::<T>()
                )
            });

        bevy::prelude::debug!("AnySendNamedBox'ed '{}' downcasted to '{}'", self.name, std::any::type_name::<T>());

        value
    }

    pub fn inner_ref<T: Any>(&self) -> &T {
        let value = self.inner
            .downcast_ref()
            .unwrap_or_else(|| {
                unreachable!("Failed to downcast AnySendNamedBox from '{}' to '{}'",
                    self.name,
                    std::any::type_name::<T>()
                )
            });

        bevy::prelude::debug!("AnySendNamedBox'ed '{}' downcasted to '{}'", self.name, std::any::type_name::<T>());

        value
    }

    pub fn inner_mut<T: Any>(&mut self) -> &mut T {
        let value = self.inner
            .downcast_mut()
            .unwrap_or_else(|| {
                unreachable!("Failed to downcast AnySendNamedBox from '{}' to '{}'",
                    self.name,
                    std::any::type_name::<T>()
                )
            });

        bevy::prelude::debug!("AnySendNamedBox'ed '{}' downcasted to '{}'", self.name, std::any::type_name::<T>());

        value
    }
}

pub struct AnySendSyncNamedBox {
    name: String,
    inner: Box<dyn Any + Send + Sync>,
}

impl AnySendSyncNamedBox {
    pub fn new<T: Any + Send + Sync + 'static>(value: T, name: String) -> Self {
        bevy::prelude::debug!("AnySendSyncNamedBox'ed '{}'", name);

        Self {
            name,
            inner: Box::new(value),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn into_inner<T: Any>(self) -> T {
        let value = *self.inner
            .downcast()
            .unwrap_or_else(|_| {
                unreachable!("Failed to downcast AnySendSyncNamedBox from '{}' to '{}'",
                    self.name,
                    std::any::type_name::<T>()
                )
            });

        bevy::prelude::debug!("AnySendSyncNamedBox'ed '{}' downcasted to '{}'", self.name, std::any::type_name::<T>());

        value
    }

    pub fn inner_ref<T: Any>(&self) -> &T {
        let value = self.inner
            .downcast_ref()
            .unwrap_or_else(|| {
                unreachable!("Failed to downcast AnySendSyncNamedBox from '{}' to '{}'",
                    self.name,
                    std::any::type_name::<T>()
                )
            });

        bevy::prelude::debug!("AnySendSyncNamedBox'ed '{}' downcasted to '{}'", self.name, std::any::type_name::<T>());

        value
    }

    pub fn inner_mut<T: Any>(&mut self) -> &mut T {
        let value = self.inner
            .downcast_mut()
            .unwrap_or_else(|| {
                unreachable!("Failed to downcast AnySendSyncNamedBox from '{}' to '{}'",
                    self.name,
                    std::any::type_name::<T>()
                )
            });

        bevy::prelude::debug!("AnySendSyncNamedBox'ed '{}' downcasted to '{}'", self.name, std::any::type_name::<T>());

        value
    }
}
