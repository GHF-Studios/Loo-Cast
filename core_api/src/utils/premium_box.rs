use bevy::prelude::{warn, Reflect};
use std::any::Any;
use std::fmt::Debug;
use std::panic::Location;

use crate::config::statics::CONFIG;

#[cfg(feature = "allow_premium_box")]
#[derive(Reflect)]
pub struct AnySendSyncPremiumBox {
    name: String,
    #[reflect(ignore, default = "placeholder_inner")]
    inner: Box<dyn Any + Send + Sync>,
    #[reflect(ignore, default = "placeholder_callsite")]
    src_call_site: &'static Location<'static>,
}

#[cfg(feature = "allow_premium_box")]
impl Debug for AnySendSyncPremiumBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AnySendSyncPremiumBox(name: {}, src_call_site: {})", self.name, self.src_call_site)
    }
}

#[cfg(feature = "allow_premium_box")]
impl AnySendSyncPremiumBox {
    #[track_caller]
    pub fn new<T: Any + Send + Sync + 'static>(value: T, name: String) -> Self {
        Self {
            name,
            inner: Box::new(value),
            src_call_site: Location::caller(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    #[track_caller]
    pub fn into_inner<T: Any>(self) -> T {
        let src_call_site = self.src_call_site;
        let dest_call_site = Location::caller();

        let inner = *self.inner.downcast().unwrap_or_else(|_| {
            panic!(
                "Failed to downcast AnySendSyncPremiumBox: {{\n\tSrc:\t\t'{}' \n\tSrcType:\t'{}' \n\tDest:\t\t'{}' \n\tDestType:\t'{}'\n}}",
                src_call_site,
                self.name,
                dest_call_site,
                std::any::type_name::<T>(),
            )
        });

        if CONFIG().get("utils/premium_box/success_logging_enabled") {
            warn!(
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

        let inner = self.inner.downcast_ref().unwrap_or_else(|| {
            panic!(
                "Failed to downcast AnySendSyncPremiumBox: {{\n\tSrc:\t\t'{}' \n\tSrcType:\t'{}' \n\tDest:\t\t'{}' \n\tDestType:\t'{}'\n}}",
                src_call_site,
                self.name,
                dest_call_site,
                std::any::type_name::<T>(),
            )
        });

        if CONFIG().get("utils/premium_box/success_logging_enabled") {
            warn!(
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

        let inner = self.inner.downcast_mut().unwrap_or_else(|| {
            panic!(
                "Failed to downcast AnySendSyncPremiumBox: {{\n\tSrc:\t\t'{}' \n\tSrcType:\t'{}' \n\tDest:\t\t'{}' \n\tDestType:\t'{}'\n}}",
                src_call_site,
                self.name,
                dest_call_site,
                std::any::type_name::<T>(),
            )
        });

        if CONFIG().get("utils/premium_box/success_logging_enabled") {
            warn!(
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

#[cfg(feature = "allow_premium_box")]
fn placeholder_inner() -> Box<dyn Any + Send + Sync> {
    Box::new(())
}

#[cfg(feature = "allow_premium_box")]
fn placeholder_callsite() -> &'static Location<'static> {
    Location::caller()
}

#[cfg(not(feature = "allow_premium_box"))]
#[repr(transparent)]
pub struct AnySendSyncPremiumBox {
    inner: Box<dyn Any + Send + Sync>,
}

#[cfg(not(feature = "allow_premium_box"))]
impl Debug for AnySendSyncPremiumBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AnySendSyncPremiumBox(Premium Info disabled by feature flag!)")
    }
}

#[cfg(not(feature = "allow_premium_box"))]
impl AnySendSyncPremiumBox {
    pub fn new<T: Any + Send + Sync + 'static>(value: T, _name: String) -> Self {
        Self { inner: Box::new(value) }
    }

    pub fn name(&self) -> &str {
        "<Premium Info disabled by feature flag!>"
    }

    pub fn into_inner<T: Any>(self) -> T {
        *self
            .inner
            .downcast()
            .unwrap_or_else(|_| panic!("Failed to downcast AnySendSyncPremiumBox: {{ Premium Info disabled by feature flag! }}"))
    }

    pub fn inner_ref<T: Any>(&self) -> &T {
        self.inner
            .downcast_ref()
            .unwrap_or_else(|| panic!("Failed to downcast AnySendSyncPremiumBox: {{ Premium Info disabled by feature flag! }}"))
    }

    pub fn inner_mut<T: Any>(&mut self) -> &mut T {
        self.inner
            .downcast_mut()
            .unwrap_or_else(|| panic!("Failed to downcast AnySendSyncPremiumBox: {{ Premium Info disabled by feature flag! }}"))
    }
}
