use std::any::Any;
use std::panic::Location;
use bevy::prelude::info;

const SUCCESS_INFO_LOGGING: bool = false;

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
        
        let inner = *self
            .inner
            .downcast()
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to downcast AnySendPremiumBox: {{\n\tSrc:\t\t'{}' \n\tSrcType:\t'{}' \n\tDest:\t\t'{}' \n\tDestType:\t'{}'\n}}",
                    src_call_site,
                    self.name,
                    dest_call_site,
                    std::any::type_name::<T>(),
                )
            });

        if SUCCESS_INFO_LOGGING {
            info!(
                "Succeeded to downcast AnySendPremiumBox: {{\n\tSrc:\t\t'{}' \n\tSrcType:\t'{}' \n\tDest:\t\t'{}' \n\tDestType:\t'{}'\n}}",
                src_call_site,
                self.name,
                dest_call_site,
                std::any::type_name::<T>(),
            );
        }

        inner
    }

    #[track_caller]
    pub fn inner_ref<T: Any>(&self) -> &T {
        let src_call_site = self.src_call_site;
        let dest_call_site = Location::caller();

        let inner = self.inner
            .downcast_ref()
            .unwrap_or_else(|| {
                panic!(
                    "Failed to downcast AnySendPremiumBox: {{\n\tSrc:\t\t'{}' \n\tSrcType:\t'{}' \n\tDest:\t\t'{}' \n\tDestType:\t'{}'\n}}",
                    src_call_site,
                    self.name,
                    dest_call_site,
                    std::any::type_name::<T>(),
                )
            });

        if SUCCESS_INFO_LOGGING {
            info!(
                "Succeeded to downcast AnySendPremiumBox: {{\n\tSrc:\t\t'{}' \n\tSrcType:\t'{}' \n\tDest:\t\t'{}' \n\tDestType:\t'{}'\n}}",
                src_call_site,
                self.name,
                dest_call_site,
                std::any::type_name::<T>(),
            );
        }

        inner
    }

    #[track_caller]
    pub fn inner_mut<T: Any>(&mut self) -> &mut T {
        let src_call_site = self.src_call_site;
        let dest_call_site = Location::caller();

        let inner = self.inner
            .downcast_mut()
            .unwrap_or_else(|| {
                panic!(
                    "Failed to downcast AnySendPremiumBox: {{\n\tSrc:\t\t'{}' \n\tSrcType:\t'{}' \n\tDest:\t\t'{}' \n\tDestType:\t'{}'\n}}",
                    src_call_site,
                    self.name,
                    dest_call_site,
                    std::any::type_name::<T>(),
                )
            });

        if SUCCESS_INFO_LOGGING {
            info!(
                "Succeeded to downcast AnySendPremiumBox: {{\n\tSrc:\t\t'{}' \n\tSrcType:\t'{}' \n\tDest:\t\t'{}' \n\tDestType:\t'{}'\n}}",
                src_call_site,
                self.name,
                dest_call_site,
                std::any::type_name::<T>(),
            );
        }

        inner
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

        let inner = *self.inner
            .downcast()
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to downcast AnySendSyncPremiumBox: {{\n\tSrc:\t\t'{}' \n\tSrcType:\t'{}' \n\tDest:\t\t'{}' \n\tDestType:\t'{}'\n}}",
                    src_call_site,
                    self.name,
                    dest_call_site,
                    std::any::type_name::<T>(),
                )
            }
        );

        if SUCCESS_INFO_LOGGING {
            info!(
                "Succeeded to downcast AnySendSyncPremiumBox: {{\n\tSrc:\t\t'{}' \n\tSrcType:\t'{}' \n\tDest:\t\t'{}' \n\tDestType:\t'{}'\n}}",
                src_call_site,
                self.name,
                dest_call_site,
                std::any::type_name::<T>(),
            );
        }

        inner
    }

    #[track_caller]
    pub fn inner_ref<T: Any>(&self) -> &T {
        let src_call_site = self.src_call_site;
        let dest_call_site = Location::caller();

        let inner = self.inner
            .downcast_ref()
            .unwrap_or_else(|| {
                panic!(
                    "Failed to downcast AnySendSyncPremiumBox: {{\n\tSrc:\t\t'{}' \n\tSrcType:\t'{}' \n\tDest:\t\t'{}' \n\tDestType:\t'{}'\n}}",
                    src_call_site,
                    self.name,
                    dest_call_site,
                    std::any::type_name::<T>(),
                )
            }
        );

        if SUCCESS_INFO_LOGGING {
            info!(
                "Succeeded to downcast AnySendSyncPremiumBox: {{\n\tSrc:\t\t'{}' \n\tSrcType:\t'{}' \n\tDest:\t\t'{}' \n\tDestType:\t'{}'\n}}",
                src_call_site,
                self.name,
                dest_call_site,
                std::any::type_name::<T>(),
            );
        }

        inner
    }

    #[track_caller]
    pub fn inner_mut<T: Any>(&mut self) -> &mut T {
        let src_call_site = self.src_call_site;
        let dest_call_site = Location::caller();

        let inner = self.inner
            .downcast_mut()
            .unwrap_or_else(|| {
                panic!(
                    "Failed to downcast AnySendSyncPremiumBox: {{\n\tSrc:\t\t'{}' \n\tSrcType:\t'{}' \n\tDest:\t\t'{}' \n\tDestType:\t'{}'\n}}",
                    src_call_site,
                    self.name,
                    dest_call_site,
                    std::any::type_name::<T>(),
                )
            }
        );

        if SUCCESS_INFO_LOGGING {
            info!(
                "Succeeded to downcast AnySendSyncPremiumBox: {{\n\tSrc:\t\t'{}' \n\tSrcType:\t'{}' \n\tDest:\t\t'{}' \n\tDestType:\t'{}'\n}}",
                src_call_site,
                self.name,
                dest_call_site,
                std::any::type_name::<T>(),
            );
        }

        inner
    }
}
