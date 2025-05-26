use std::any::Any;

pub struct DebugAnySendBox {
    name: String,
    inner: Box<dyn Any + Send>,
}

impl DebugAnySendBox {
    pub fn new<T: Any + Send + 'static>(value: T, name: String) -> Self {
        Self {
            name,
            inner: Box::new(value),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.inner.downcast_ref()
    }

    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.inner.downcast_mut()
    }

    pub fn into_inner<T: Any>(self) -> Result<Box<T>, Box<dyn Any + Send>> {
        self.inner.downcast()
    }
}

pub struct DebugAnySendSyncBox {
    name: String,
    inner: Box<dyn Any + Send + Sync>,
}

impl DebugAnySendSyncBox {
    pub fn new<T: Any + Send + Sync + 'static>(value: T, name: String) -> Self {
        Self {
            name,
            inner: Box::new(value),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.inner.downcast_ref()
    }

    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.inner.downcast_mut()
    }

    pub fn into_inner<T: Any>(self) -> Result<Box<T>, Box<dyn Any + Send + Sync>> {
        self.inner.downcast()
    }
}
