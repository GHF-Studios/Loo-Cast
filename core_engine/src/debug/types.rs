use std::any::Any;

pub struct AnySendNamedBox {
    name: String,
    inner: Box<dyn Any + Send>,
}

impl AnySendNamedBox {
    pub fn new<T: Any + Send + 'static>(value: T, name: String) -> Self {
        Self {
            name,
            inner: Box::new(value),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn downcast_ref<T: Any>(&self) -> &T {
        self.inner.downcast_ref().unwrap_or_else(|| panic!("Failed to downcast AnyNamedBox from '{}' to '{}'",
                self.name,
                std::any::type_name::<T>()))
    }

    pub fn downcast_mut<T: Any>(&mut self) -> &mut T {
        self.inner.downcast_mut().unwrap_or_else(|| panic!("Failed to downcast AnyNamedBox from '{}' to '{}'",
                self.name,
                std::any::type_name::<T>()))
    }

    pub fn into_inner<T: Any>(self) -> T {
        *self.inner.downcast().unwrap_or_else(|_| panic!("Failed to downcast AnyNamedBox from '{}' to '{}'",
                self.name,
                std::any::type_name::<T>()))
    }
}

pub struct AnySendSyncNamedBox {
    name: String,
    inner: Box<dyn Any + Send + Sync>,
}

impl AnySendSyncNamedBox {
    pub fn new<T: Any + Send + Sync + 'static>(value: T, name: String) -> Self {
        Self {
            name,
            inner: Box::new(value),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn downcast_ref<T: Any>(&self) -> &T {
        self.inner.downcast_ref().unwrap_or_else(|| panic!("Failed to downcast AnyNamedBox from '{}' to '{}'",
                self.name,
                std::any::type_name::<T>()))
    }

    pub fn downcast_mut<T: Any>(&mut self) -> &mut T {
        self.inner.downcast_mut().unwrap_or_else(|| panic!("Failed to downcast AnyNamedBox from '{}' to '{}'",
                self.name,
                std::any::type_name::<T>()))
    }

    pub fn into_inner<T: Any>(self) -> T {
        *self.inner.downcast().unwrap_or_else(|_| panic!("Failed to downcast AnyNamedBox from '{}' to '{}'",
                self.name,
                std::any::type_name::<T>()))
    }
}
