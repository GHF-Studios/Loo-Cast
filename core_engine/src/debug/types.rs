use std::any::Any;
use std::panic::Location;

pub struct AnySendPremiumBox {
    name: String,
    inner: Box<dyn Any + Send>,
    src_call_site: &'static Location<'static>,
}

impl AnySendPremiumBox {
    #[track_caller]
    pub fn new<T: Any + Send + 'static>(value: T, name: String) -> Self {
        Self { name, inner: Box::new(value), src_call_site: Location::caller() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    #[track_caller]
    pub fn into_inner<T: Any>(self) -> T {
        let src_call_site = self.src_call_site;
        let dest_call_site = Location::caller();
        
        *self
            .inner
            .downcast()
            .unwrap_or_else(|_| {
                unreachable!(
                    "Failed to downcast AnySendPremiumBox(SRC '{}' -> DEST '{}') from SrcType '{}' to DestType '{}'",
                    src_call_site,
                    dest_call_site,
                    self.name,
                    std::any::type_name::<T>()
                )
            })
    }

    #[track_caller]
    pub fn inner_ref<T: Any>(&self) -> &T {
        let src_call_site = self.src_call_site;
        let dest_call_site = Location::caller();

        self.inner
            .downcast_ref()
            .unwrap_or_else(|| {
                unreachable!(
                    "Failed to downcast AnySendPremiumBox(SRC '{}' -> DEST '{}') from SrcType '{}' to DestType '{}'",
                    src_call_site,
                    dest_call_site,
                    self.name,
                    std::any::type_name::<T>()
                )
            })
    }

    #[track_caller]
    pub fn inner_mut<T: Any>(&mut self) -> &mut T {
        let src_call_site = self.src_call_site;
        let dest_call_site = Location::caller();

        self.inner
            .downcast_mut()
            .unwrap_or_else(|| {
                unreachable!(
                    "Failed to downcast AnySendPremiumBox(SRC '{}' -> DEST '{}') from SrcType '{}' to DestType '{}'",
                    src_call_site,
                    dest_call_site,
                    self.name,
                    std::any::type_name::<T>()
                )
            })
    }
}

pub struct AnySendSyncPremiumBox {
    name: String,
    inner: Box<dyn Any + Send + Sync>,
    src_call_site: &'static Location<'static>,
}

impl AnySendSyncPremiumBox {
    #[track_caller]
    pub fn new<T: Any + Send + Sync + 'static>(value: T, name: String) -> Self {
        Self { name, inner: Box::new(value), src_call_site: Location::caller() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    #[track_caller]
    pub fn into_inner<T: Any>(self) -> T {
        let src_call_site = self.src_call_site;
        let dest_call_site = Location::caller();

        *self.inner.downcast().unwrap_or_else(|_| {
            unreachable!(
                "Failed to downcast AnySendSyncPremiumBox(SRC '{}' -> DEST '{}') from SrcType '{}' to DestType '{}'",
                src_call_site,
                dest_call_site,
                self.name,
                std::any::type_name::<T>()
            )
        })
    }

    #[track_caller]
    pub fn inner_ref<T: Any>(&self) -> &T {
        let src_call_site = self.src_call_site;
        let dest_call_site = Location::caller();

        self.inner.downcast_ref().unwrap_or_else(|| {
            unreachable!(
                "Failed to downcast AnySendSyncPremiumBox(SRC '{}' -> DEST '{}') from SrcType '{}' to DestType '{}'",
                src_call_site,
                dest_call_site,
                self.name,
                std::any::type_name::<T>()
            )
        })
    }

    #[track_caller]
    pub fn inner_mut<T: Any>(&mut self) -> &mut T {
        let src_call_site = self.src_call_site;
        let dest_call_site = Location::caller();

        self.inner.downcast_mut().unwrap_or_else(|| {
            unreachable!(
                "Failed to downcast AnySendSyncPremiumBox(SRC '{}' -> DEST '{}') from SrcType '{}' to DestType '{}'",
                src_call_site,
                dest_call_site,
                self.name,
                std::any::type_name::<T>()
            )
        })
    }
}
