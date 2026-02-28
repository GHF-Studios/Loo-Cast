#![allow(clippy::missing_safety_doc)]

use once_cell::sync::Lazy;
use rhai::{ImmutableString, RhaiNativeFunc, Shared, Variant};
use std::hash::Hash;
use std::sync::Arc;

use crate::reflection::traits::StaticTraitObject;
use crate::utils::clone_closure::{ApplyCloneClosure, CloneClosure};
use crate::utils::clone_lazy::CloneLazy;


// TODO: Consolidate Type::name() into here as TYPE_NAME similarly to GetTraitId
// TODO: Add string-format documentation or newtype with invariant-enforcing on construction
pub trait GetTypeId: Sized + 'static {
    const TYPE_ID: &'static str;
}

// TODO: Add string-format documentation or newtype with invariant-enforcing on construction
pub trait GetTraitName: Clone + Sized + 'static {
    const TRAIT_NAME: &'static str;
}
pub trait GetTraitObjectName: Clone + Sized + 'static {
    const TRAIT_OBJECT_NAME: &'static str;
}
pub trait DynGetTraitName: 'static {
    fn trait_name(&self) -> &'static str;
}
pub trait DynGetTraitObjectName: 'static {
    fn trait_object_name(&self) -> &'static str;
}
pub trait GetTraitId: Clone + Sized + 'static {
    const TRAIT_ID: &'static str;
}
pub trait GetTraitObjectId: Clone + Sized + 'static {
    const TRAIT_OBJECT_ID: &'static str;
}
pub trait ToTraitObject<T: GetTraitId>: Sized {
    fn cast_to(self) -> StaticTraitObject<T>;
    fn cast_from(obj: StaticTraitObject<T>) -> Self;
}


use crate::reflection::{
    ids::TypeId,
    type_info::TypeInfo,
};
use crate::reflection::internals::statics::{CTOR_REGISTRY, METHOD_REGISTRY, RAW_REFLECTION_METADATA, STATIC_FUNCTION_REGISTRY, TYPE_REGISTRY};
use crate::script::access::ScopedAccessHandle;
use crate::utils::string::*;
use std::any::Any;

/// Provides read-only, non-mutating access to a value of type `T` from `Self`,
/// typically used to expose internal state to external systems (e.g., scripting).
pub(crate) trait ReadAccessProvider<T: Clone> {
    /// Returns a clone of a value of type `T`, by invoking a named method with arguments.
    ///
    /// This access does *not* mutate `self`.
    ///
    /// - `method`: A string identifying the access method to invoke.
    /// - `args`: Arbitrary arguments, passed as a boxed `Any`.
    fn access(&self, method: &str, args: Box<dyn Any>) -> T;
}

/// Provides mutable access to a value of type `T` from `Self`,
/// allowing state mutation via method-like invocation.
pub(crate) trait WriteAccessProvider<T: Clone> {
    /// Returns a clone of a value of type `T`, by invoking a named method with arguments.
    ///
    /// This access *may* mutate `self`.
    ///
    /// - `method`: A string identifying the access method to invoke.
    /// - `args`: Arbitrary arguments, passed as a boxed `Any`.
    fn access(&mut self, method: &str, args: Box<dyn Any>) -> T;
}

/// Grants **temporally-scoped, mutable** access to a value of type `T` from `Self`,
/// using a named method and optional dynamic arguments.
///
/// Unlike `WriteAccessProvider`, this trait enables *borrowing* the data via a scoped handle,
/// rather than cloning it. It is intended for cases where external systems (e.g., scripting or dynamic plugins)
/// need transient, direct access to internal data.
///
/// # Safety
/// This trait is `unsafe` because it may internally manipulate lifetimes (e.g., coercing to `'static`)
/// in order to integrate with systems that require erased or delayed access semantics.
/// 
/// ## Contract
/// - `start_access` and `end_access` **must** be called during the **same execution of the same Bevy system**.
/// - The returned `ScopedAccessHandle<T>` must **not escape** the scope in which `start_access` was called.
/// - `end_access` **must be called** before the system yields control back to Bevy's ECS scheduler.
/// - The access must remain **synchronous, non-blocking, and locally scoped**—no async, no deferring, no caching handles.
///
/// These constraints ensure that Bevy's borrowing rules remain intact and that temporary borrows
/// are properly released before Bevy resumes world access.
///
/// Implementors must ensure that the handle returned from `start_access` represents a valid,
/// non-aliasing borrow for the lifetime of that access, and that `end_access` reliably releases it.
///
/// Violating these guarantees may result in **undefined behavior**, including use-after-free or aliasing mutable borrows.
/// # "Design rationale":
/// This trait provides unsafe, dynamic, lifetime-erased access across a constrained, synchronous borrow window.
/// Because Bevy controls world borrowing tightly, ScopedAccessHandle<T> must act like a scoped guard.
/// That means the full access cycle (start -> use -> end) MUST complete within a single ECS system frame.
/// Don't try to store handles, yield them across frames, or wrap this in async — it will break Rust's safety model.
pub(crate) unsafe trait ScopedAccessProvider<T> {
    /// Begins a scoped, synchronous access to a value of type `T` from `Self`, using a named method and arguments.
    /// Returns a handle representing the active borrow.
    ///
    /// # Safety
    /// The returned handle must not escape the calling system. This method must be followed by a call to `end_access`
    /// during the same system execution before control returns to Bevy.
    unsafe fn start_access(&mut self, method: &str, args: Box<dyn Any>) -> ScopedAccessHandle<T>;

    /// Ends a previously started scoped access, releasing the associated borrow.
    ///
    /// # Safety
    /// This must only be called with a handle previously returned by `start_access`
    /// during the current system execution.
    unsafe fn end_access(&mut self, handle: ScopedAccessHandle<T>);
}

// The vision: 

// The actual end-user code
// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< //
pub mod shop {
    use core_mod_macros::reflect_top_level_module;
    use once_cell::sync::Lazy;

    use crate::{reflection::internals::traits::*, utils::clone_closure::CloneClosure};

    reflect_top_level_module!(
        id = shop,
        sub_modules = [divisions],
        traits = [],
        types = [],
        module_associated_functions = [],
    );
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> //

// Expanded MetaProgramming Magic (It's just a bunch of metadata)
// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< //
    #[allow(non_upper_case_globals)]
    static __SHOP__TOP_LEVEL_MODULE__: CloneLazy<TopLevelModuleMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __Shop__TopLevelModule__.from_comptime_to_runtime(&__Shop__TopLevelModule__)));
    inventory::submit!(TopLevelModuleMetadataEntry(&__SHOP__TOP_LEVEL_MODULE__));

    #[allow(non_camel_case_types)]
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct __Shop__TopLevelModule__;
    impl ConstDynMetadata for __Shop__TopLevelModule__ {
        fn raw_rust_module_path(&self) -> &'static str { module_path!() }
    }
    impl NativeModuleConstDynMetadata for __Shop__TopLevelModule__ {
        fn traits(&self) -> CloneLazy<Vec<TraitPath>> {
            CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
        }
        fn types(&self) -> CloneLazy<Vec<TypePath>> {
            CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
        }
        fn inherent_impls(&self) -> CloneLazy<Vec<InherentImplPath>> {
            CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
        }
        fn trait_impls(&self) -> CloneLazy<Vec<TraitImplPath>> {
            CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
        }
    }
    impl TopLevelModuleConstDynMetadata for __Shop__TopLevelModule__ {
        fn id_path(&self) -> CloneLazy<TopLevelModulePath> {
            CloneLazy::new(CloneClosure::new((), |_, _| "shop".into()))
        }
        fn sub_modules(&self) -> CloneLazy<Vec<SubModulePath>> {
            CloneLazy::new(CloneClosure::new((), |_, _| vec!["shop::divisions".into()]))
        }
        fn type_proxy_modules(&self) -> CloneLazy<Vec<TypeProxyModulePath>> {
            CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
        }
        fn module_associated_functions(&self) -> CloneLazy<Vec<ModuleAssociatedFunctionPath>> {
            CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
        }
    }
    impl TopLevelModuleDynamicTypedMetadata for __Shop__TopLevelModule__ {}
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> //

// The actual end-user code
// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< //
    pub mod divisions {
        use core_mod_macros::reflect_sub_module;
        use once_cell::sync::Lazy;

        use crate::{reflection::internals::traits::*, utils::clone_closure::CloneClosure};

        reflect_sub_module!(
            id = shop::divisions,
            sub_modules = [sex],
            traits = [],
            types = [],
            module_associated_functions = [],
        );
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> //

// Expanded MetaProgramming Magic (It's just a bunch of metadata)
// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< //
        #[allow(non_upper_case_globals)]
        static __DIVISIONS__SUB_MODULE__: CloneLazy<SubModuleMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __Divisions__SubModule__.from_comptime_to_runtime(&__Divisions__SubModule__)));
        inventory::submit!(SubModuleMetadataEntry(&__DIVISIONS__SUB_MODULE__));

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub struct __Divisions__SubModule__;
        impl ConstDynMetadata for __Divisions__SubModule__ {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }
        impl NativeModuleConstDynMetadata for __Divisions__SubModule__ {
            fn traits(&self) -> CloneLazy<Vec<TraitPath>> {
                CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
            }
            fn types(&self) -> CloneLazy<Vec<TypePath>> {
                CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
            }
            fn inherent_impls(&self) -> CloneLazy<Vec<InherentImplPath>> {
                CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
            }
            fn trait_impls(&self) -> CloneLazy<Vec<TraitImplPath>> {
                CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
            }
        }
        impl SubModuleConstDynMetadata for __Divisions__SubModule__ {
            fn id_path(&self) -> CloneLazy<SubModulePath> {
                CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions".into()))
            }
            fn sub_modules(&self) -> CloneLazy<Vec<SubModulePath>> {
                CloneLazy::new(CloneClosure::new((), |_, _| vec!["shop::divisions::sex".into()]))
            }
            fn type_proxy_modules(&self) -> CloneLazy<Vec<TypeProxyModulePath>> {
                CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
            }
            fn module_associated_functions(&self) -> CloneLazy<Vec<ModuleAssociatedFunctionPath>> {
                CloneLazy::new(CloneClosure::new((), |_, _| vec![]))
            }
        }
        impl SubModuleDynamicTypedMetadata for __Divisions__SubModule__ {}
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> //

// The actual end-user code
// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< //
        pub mod sex {
            use core_mod_macros::{
                reflect_sub_module,
                reflect_trait,
                reflect_trait_impl,
                reflect_type,
                reflect_inherent_impl,
                reflect_module_associated_function,
                reflect_item_associated_function,
                reflect_constructor_function,
                reflect_method_function
            };
            use once_cell::sync::Lazy;
            use rhai::ImmutableString;

            use crate::{reflection::internals::traits::*, utils::{clone_closure::CloneClosure}};

            reflect_sub_module!(
                id_path = shop::divisions::sex,
                sub_modules = [],
                traits = [SexShopTest],
                types = [SexShopProduct],
                module_associated_functions = [test_function],
            );

            #[reflect_trait(shop::divisions::sex::SexShopTest)]
            pub trait SexShopTest {
                fn test();
            }
            
            #[reflect_type(shop::divisions::sex::SexShopProduct)]
            #[derive(Clone)]
            pub struct SexShopProduct {
                name: &'static str,
                price_usd: f32,
            }
            #[reflect_inherent_impl(shop::divisions::sex::SexShopProduct)]
            impl SexShopProduct {
                #[reflect_constructor_function(shop::divisions::sex::SexShopProduct)]
                pub fn new(name: &'static str, price_usd: f32) -> Self { Self { name, price_usd } }

                #[reflect_method_function(shop::divisions::sex::SexShopProduct)]
                pub fn name(&self) -> &'static str { self.name }

                #[reflect_method_function(shop::divisions::sex::SexShopProduct)]
                pub fn price_usd(&self) -> f32 { self.price_usd }

                #[reflect_item_associated_function(shop::divisions::sex::SexShopProduct)]
                pub fn verify_price(price_usd: f32) -> Result<(), ()> {
                    if price_usd >= 0.0 { Ok(()) } else { Err(()) }
                }
            }
            #[reflect_trait_impl(<shop::divisions::sex::SexShopProduct as shop::divisions::sex::SexShopTest>)]
            impl SexShopTest for SexShopProduct {
                #[reflect_item_associated_function(<shop::divisions::sex::SexShopProduct as shop::divisions::sex::SexShopTest>)]
                fn test() {
                    println!("Small banana sound!")
                }
            }

            #[reflect_module_associated_function(shop::divisions::sex::test_function)]
            pub fn test_function() {
                println!("Big paling sound!")
            }
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> //

// Expanded MetaProgramming Magic (It's just a bunch of metadata)
// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< //

            #[allow(non_upper_case_globals)]
            static __SEX__SUB_MODULE__: CloneLazy<SubModuleMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __Sex__SubModule__.from_comptime_to_runtime(&__Sex__SubModule__)));
            inventory::submit!(SubModuleMetadataEntry(&__SEX__SUB_MODULE__));
            #[allow(non_upper_case_globals)]
            static __SEX_SHOP_TEST__TRAIT__: CloneLazy<TraitMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __SexShopTest__Trait__.from_comptime_to_runtime(&__SexShopTest__Trait__)));
            inventory::submit!(TraitMetadataEntry(&__SEX_SHOP_TEST__TRAIT__));
            #[allow(non_upper_case_globals)]
            static __SEX_SHOP_TEST__TRAIT_OBJECT__: CloneLazy<TraitObjectMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __SexShopTest__TraitObject__.from_comptime_to_runtime(&__SexShopTest__TraitObject__)));
            inventory::submit!(TraitObjectMetadataEntry(&__SEX_SHOP_TEST__TRAIT_OBJECT__));
            #[allow(non_upper_case_globals)]
            static __SEX_SHOP_PRODUCT__TYPE__: CloneLazy<TypeMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __SexShopProduct__Type__.from_comptime_to_runtime(&__SexShopProduct__Type__)));
            inventory::submit!(TypeMetadataEntry(&__SEX_SHOP_PRODUCT__TYPE__));
            #[allow(non_upper_case_globals)]
            static __TEST_FUNCTION__MODULE_ASSOCIATED_FUNCTION__: CloneLazy<ModuleAssociatedFunctionMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __TestFunction__ModuleAssociatedFunction__.from_comptime_to_runtime(&__TestFunction__ModuleAssociatedFunction__)));
            inventory::submit!(ModuleAssociatedFunctionMetadataEntry(&__TEST_FUNCTION__MODULE_ASSOCIATED_FUNCTION__));
            #[allow(non_upper_case_globals)]
            static __VERIFY_PRICE__ITEM_ASSOCIATED_FUNCTION__: CloneLazy<ItemAssociatedFunctionMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __VerifyPrice__ItemAssociatedFunction__.from_comptime_to_runtime(&__VerifyPrice__ItemAssociatedFunction__)));
            inventory::submit!(ItemAssociatedFunctionMetadataEntry(&__VERIFY_PRICE__ITEM_ASSOCIATED_FUNCTION__));
            #[allow(non_upper_case_globals)]
            static __NEW__CONSTRUCTOR_FUNCTION__: CloneLazy<ConstructorFunctionMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __New__ConstructorFunction__.from_comptime_to_runtime(&__New__ConstructorFunction__)));
            inventory::submit!(ConstructorFunctionMetadataEntry(&__NEW__CONSTRUCTOR_FUNCTION__));
            #[allow(non_upper_case_globals)]
            static __NAME__METHOD_FUNCTION__: CloneLazy<MethodFunctionMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __Name__MethodFunction__.from_comptime_to_runtime(&__Name__MethodFunction__)));
            inventory::submit!(MethodFunctionMetadataEntry(&__NAME__METHOD_FUNCTION__));
            #[allow(non_upper_case_globals)]
            static __PRICE_USD__METHOD_FUNCTION__: CloneLazy<MethodFunctionMetadata> = CloneLazy::new(CloneClosure::new((), |(), ()| __PriceUsd__MethodFunction__.from_comptime_to_runtime(&__PriceUsd__MethodFunction__)));
            inventory::submit!(MethodFunctionMetadataEntry(&__PRICE_USD__METHOD_FUNCTION__));

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __Sex__SubModule__;
            impl ConstDynMetadata for __Sex__SubModule__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl NativeModuleConstDynMetadata for __Sex__SubModule__ {
                fn traits(&self) -> CloneLazy<Vec<TraitPath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec!["shop::divisions::sex::SexShopTest".into()])) }
                fn types(&self) -> CloneLazy<Vec<TypePath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec!["shop::divisions::sex::SexShopProduct".into()])) }
                fn inherent_impls(&self) -> CloneLazy<Vec<InherentImplPath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec![])) }
                fn trait_impls(&self) -> CloneLazy<Vec<TraitImplPath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec![])) }
            }
            impl SubModuleConstDynMetadata for __Sex__SubModule__ {
                fn id_path(&self) -> CloneLazy<SubModulePath> { CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions::sex".into())) }
                fn sub_modules(&self) -> CloneLazy<Vec<SubModulePath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec![])) }
                fn type_proxy_modules(&self) -> CloneLazy<Vec<TypeProxyModulePath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec![])) }
                fn module_associated_functions(&self) -> CloneLazy<Vec<ModuleAssociatedFunctionPath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec!["shop::divisions::sex::test_function".into()])) }
            }
            impl SubModuleDynamicTypedMetadata for __Sex__SubModule__ {}

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __SexShopTest__Trait__;
            impl ConstDynMetadata for __SexShopTest__Trait__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl DynGetTraitName for __SexShopTest__Trait__ {
                fn trait_name(&self) -> &'static str { "SexShopTest" }
            }
            impl GetTraitId for __SexShopTest__Trait__ {
                const TRAIT_ID: &'static str = "shop::divisions::sex::SexShopTest";
            }
            impl TraitConstDynMetadata for __SexShopTest__Trait__ {
                fn id_path(&self) -> CloneLazy<TraitPath> { CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions::sex::SexShopTest".into())) }
            }
            impl TraitDynamicTypedMetadata for __SexShopTest__Trait__ {}

            // TODO: WIP! Implement properly; this is highly experimental!
            #[repr(transparent)]
            pub struct SexShopTestTraitObject(pub StaticTraitObject<__SexShopTest__Trait__>);

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __SexShopTest__TraitObject__;
            impl ConstDynMetadata for __SexShopTest__TraitObject__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl DynGetTraitObjectName for __SexShopTest__TraitObject__ {
                fn trait_object_name(&self) -> &'static str { "SexShopTestTraitObject" }
            }
            impl GetTraitObjectId for __SexShopTest__Trait__ {
                const TRAIT_OBJECT_ID: &'static str = "shop::divisions::sex::SexShopTestTraitObject";
            }
            impl TraitObjectConstDynMetadata for __SexShopTest__TraitObject__ {
                fn id_path(&self) -> CloneLazy<TraitPath> { CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions::sex::SexShopTest".into())) }
            }
            impl TraitObjectDynamicTypedMetadata for __SexShopTest__TraitObject__ {}

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __SexShopProduct__Type__;
            impl ConstDynMetadata for __SexShopProduct__Type__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl TypeConstDynMetadata for __SexShopProduct__Type__ {
                fn id_path(&self) -> CloneLazy<TypePath> { CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions::sex::SexShopProduct".into())) }
                fn method_functions(&self) -> CloneLazy<Vec<MethodFunctionPath>> {
                    CloneLazy::new(CloneClosure::new((), |_, _| vec![
                        "shop::divisions::sex::SexShopProduct::name".into(),
                        "shop::divisions::sex::SexShopProduct::price_usd".into(),
                    ]))
                }
            }
            impl TypeDynamicTypedMetadata for __SexShopProduct__Type__ {}

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __SexShopProduct__TypeProxyModule__;
            impl ConstDynMetadata for __SexShopProduct__TypeProxyModule__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl NativeModuleConstDynMetadata for __SexShopProduct__TypeProxyModule__ {
                fn traits(&self) -> CloneLazy<Vec<TraitPath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec![])) }
                fn types(&self) -> CloneLazy<Vec<TypePath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec![])) }
                fn inherent_impls(&self) -> CloneLazy<Vec<InherentImplPath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec![])) }
                fn trait_impls(&self) -> CloneLazy<Vec<TraitImplPath>> { CloneLazy::new(CloneClosure::new((), |_, _| vec![])) }
            }
            impl TypeProxyModuleConstDynMetadata for __SexShopProduct__TypeProxyModule__ {
                fn id_path(&self) -> CloneLazy<TypeProxyModulePath> { CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions::sex::SexShopProduct".into())) }
                fn item_associated_functions(&self) -> CloneLazy<Vec<ItemAssociatedFunctionPath>> {
                    CloneLazy::new(CloneClosure::new((), |_, _| vec!["shop::divisions::sex::SexShopProduct::new".into()]))
                }
                fn constructor_functions(&self) -> CloneLazy<Vec<ConstructorFunctionPath>> {
                    CloneLazy::new(CloneClosure::new((), |_, _| vec!["shop::divisions::sex::SexShopProduct::verify_price".into()]))
                }
            }
            impl TypeProxyModuleDynamicTypedMetadata for __SexShopProduct__TypeProxyModule__ {}

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __TestFunction__ModuleAssociatedFunction__;
            impl ConstDynMetadata for __TestFunction__ModuleAssociatedFunction__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl ModuleAssociatedFunctionConstDynMetadata for __TestFunction__ModuleAssociatedFunction__ {
                fn id_path(&self) -> CloneLazy<ModuleAssociatedFunctionPath> { CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions::sex::test_function".into())) }
                fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)> {
                    CloneClosure::new(self.id_path().get().function_name().clone(), |name, parent_module| {
                        rhai::FuncRegistration::new(name)
                            .set_into_module(parent_module, test_function);
                    })
                }
            }
            impl ModuleAssociatedFunctionDynamicTypedMetadata for __TestFunction__ModuleAssociatedFunction__ {}

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __VerifyPrice__ItemAssociatedFunction__;
            impl ConstDynMetadata for __VerifyPrice__ItemAssociatedFunction__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl ItemAssociatedFunctionConstDynMetadata for __VerifyPrice__ItemAssociatedFunction__ {
                fn id_path(&self) -> CloneLazy<ItemAssociatedFunctionPath> { CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions::sex::SexShopProduct::verify_price".into())) }
                fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)> {
                    CloneClosure::new(self.id_path().get().function_name().clone(), |name, parent_module| {
                        rhai::FuncRegistration::new(name)
                            .set_into_module(parent_module, SexShopProduct::verify_price);
                    })
                }
            }
            impl ItemAssociatedFunctionDynamicTypedMetadata for __VerifyPrice__ItemAssociatedFunction__ {}

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __New__ConstructorFunction__;
            impl ConstDynMetadata for __New__ConstructorFunction__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl ConstructorFunctionConstDynMetadata for __New__ConstructorFunction__ {
                fn id_path(&self) -> CloneLazy<ConstructorFunctionPath> { CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions::sex::SexShopProduct::new".into())) }
                fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)> {
                    CloneClosure::new(self.id_path().get().function_name().clone(), |name, parent_module| {
                        rhai::FuncRegistration::new(name)
                            .set_into_module(parent_module, SexShopProduct::new);
                    })
                }
            }
            impl ConstructorFunctionDynamicTypedMetadata for __New__ConstructorFunction__ {}

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __Name__MethodFunction__;
            impl ConstDynMetadata for __Name__MethodFunction__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl MethodFunctionConstDynMetadata for __Name__MethodFunction__ {
                fn id_path(&self) -> CloneLazy<MethodFunctionPath> { CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions::sex::SexShopProduct::name".into())) }
                fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Engine, (), fn(ImmutableString, &mut rhai::Engine)> {
                    CloneClosure::new(self.id_path().get().function_name().clone(), |name, engine| {
                        engine.register_fn(name, SexShopProduct::name);
                    })
                }
            }
            impl MethodFunctionDynamicTypedMetadata for __Name__MethodFunction__ {}
            
            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __PriceUsd__MethodFunction__;
            impl ConstDynMetadata for __PriceUsd__MethodFunction__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl MethodFunctionConstDynMetadata for __PriceUsd__MethodFunction__ {
                fn id_path(&self) -> CloneLazy<MethodFunctionPath> { CloneLazy::new(CloneClosure::new((), |_, _| "shop::divisions::sex::SexShopProduct::price_usd".into())) }
                fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Engine, (), fn(ImmutableString, &mut rhai::Engine)> {
                    CloneClosure::new(self.id_path().get().function_name().clone(), |name, engine| {
                        engine.register_fn(name, SexShopProduct::price_usd);
                    })
                }
            }
            impl MethodFunctionDynamicTypedMetadata for __PriceUsd__MethodFunction__ {}
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> //

// The actual end-user code
// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< //
        }
    }
}
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> //



// Metadata Entries
inventory::collect!(TopLevelModuleMetadataEntry);
pub struct TopLevelModuleMetadataEntry(pub &'static CloneLazy<TopLevelModuleMetadata>);
inventory::collect!(SubModuleMetadataEntry);
pub struct SubModuleMetadataEntry(pub &'static CloneLazy<SubModuleMetadata>);
inventory::collect!(TypeProxyModuleMetadataEntry);
pub struct TypeProxyModuleMetadataEntry(pub &'static CloneLazy<TypeProxyModuleMetadata>);
inventory::collect!(TraitMetadataEntry);
pub struct TraitMetadataEntry(pub &'static CloneLazy<TraitMetadata>);
inventory::collect!(TraitObjectMetadataEntry);
pub struct TraitObjectMetadataEntry(pub &'static CloneLazy<TraitObjectMetadata>);
inventory::collect!(TypeMetadataEntry);
pub struct TypeMetadataEntry(pub &'static CloneLazy<TypeMetadata>);
inventory::collect!(InherentImplMetadataEntry);
pub struct InherentImplMetadataEntry(pub &'static CloneLazy<InherentImplMetadata>);
inventory::collect!(TraitImplMetadataEntry);
pub struct TraitImplMetadataEntry(pub &'static CloneLazy<TraitImplMetadata>);
inventory::collect!(ModuleAssociatedFunctionMetadataEntry);
pub struct ModuleAssociatedFunctionMetadataEntry(pub &'static CloneLazy<ModuleAssociatedFunctionMetadata>);
inventory::collect!(ItemAssociatedFunctionMetadataEntry);
pub struct ItemAssociatedFunctionMetadataEntry(pub &'static CloneLazy<ItemAssociatedFunctionMetadata>);
inventory::collect!(ConstructorFunctionMetadataEntry);
pub struct ConstructorFunctionMetadataEntry(pub &'static CloneLazy<ConstructorFunctionMetadata>);
inventory::collect!(MethodFunctionMetadataEntry);
pub struct MethodFunctionMetadataEntry(pub &'static CloneLazy<MethodFunctionMetadata>);



// Module Metadata
#[derive(Clone)]
pub struct TopLevelModuleMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<TopLevelModulePath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,

    pub traits: CloneLazy<Vec<TraitPath>>,
    pub types: CloneLazy<Vec<TypePath>>,
    pub inherent_impls: CloneLazy<Vec<InherentImplPath>>,
    pub trait_impls: CloneLazy<Vec<TraitImplPath>>,

    pub sub_modules: CloneLazy<Vec<SubModulePath>>,
    pub type_proxy_modules: CloneLazy<Vec<TypeProxyModulePath>>,
    pub module_associated_functions: CloneLazy<Vec<ModuleAssociatedFunctionPath>>,
}
impl ConstDynMetadata for TopLevelModuleMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl NativeModuleConstDynMetadata for TopLevelModuleMetadata {
    fn traits(&self) -> CloneLazy<Vec<TraitPath>> { self.traits.clone() }
    fn types(&self) -> CloneLazy<Vec<TypePath>> { self.types.clone() }
    fn inherent_impls(&self) -> CloneLazy<Vec<InherentImplPath>> { self.inherent_impls.clone() }
    fn trait_impls(&self) -> CloneLazy<Vec<TraitImplPath>> { self.trait_impls.clone() }
}
impl TopLevelModuleConstDynMetadata for TopLevelModuleMetadata {
    fn id_path(&self) -> CloneLazy<TopLevelModulePath> { self.id_path.clone() }
    fn sub_modules(&self) -> CloneLazy<Vec<SubModulePath>> { self.sub_modules.clone() }
    fn type_proxy_modules(&self) -> CloneLazy<Vec<TypeProxyModulePath>> { self.type_proxy_modules.clone() }
    fn module_associated_functions(&self) -> CloneLazy<Vec<ModuleAssociatedFunctionPath>> { self.module_associated_functions.clone() }
}
impl TopLevelModuleMetadata {
    pub(super) fn register_top_level_module(&self, engine: &mut rhai::Engine) {
        let registry = RAW_REFLECTION_METADATA();
        let mut top_level_module = rhai::Module::new();
        top_level_module.set_id(self.id_path().get().module_name());

        for path in self.sub_modules().get().into_iter() {
            let sub_module = registry.sub_modules.get(&path).unwrap().clone();
            sub_module.register_sub_module(engine, &mut top_level_module);
        }

        for path in self.traits().get().into_iter() {
            let (trait_, trait_object) = registry.traits.get(&path).unwrap().clone();
            trait_.register_trait(&mut top_level_module);
            trait_object.register_trait_object(&mut top_level_module);
        }

        for path in self.types().get().into_iter() {
            let type_ = registry.types.get(&path).unwrap().clone();
            type_.register_type(engine, &mut top_level_module);
        }

        for path in self.type_proxy_modules().get().into_iter() {
            let type_proxy_module = registry.type_proxy_modules.get(&path).unwrap().clone();
            type_proxy_module.register_type_proxy_module(&mut top_level_module);
        }

        for path in self.module_associated_functions().get().into_iter() {
            let module_associated_function = registry.module_associated_functions.get(&path).unwrap().clone();
            module_associated_function.register_module_associated_function(&mut top_level_module);
        }

        engine.register_static_module(self.id_path().get().module_name(), Arc::new(top_level_module));
    }
}
#[derive(Clone)]
pub struct SubModuleMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<SubModulePath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,

    pub traits: CloneLazy<Vec<TraitPath>>,
    pub types: CloneLazy<Vec<TypePath>>,
    pub inherent_impls: CloneLazy<Vec<InherentImplPath>>,
    pub trait_impls: CloneLazy<Vec<TraitImplPath>>,

    pub sub_modules: CloneLazy<Vec<SubModulePath>>,
    pub type_proxy_modules: CloneLazy<Vec<TypeProxyModulePath>>,
    pub module_associated_functions: CloneLazy<Vec<ModuleAssociatedFunctionPath>>,
}
impl ConstDynMetadata for SubModuleMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl NativeModuleConstDynMetadata for SubModuleMetadata {
    fn traits(&self) -> CloneLazy<Vec<TraitPath>> { self.traits.clone() }
    fn types(&self) -> CloneLazy<Vec<TypePath>> { self.types.clone() }
    fn inherent_impls(&self) -> CloneLazy<Vec<InherentImplPath>> { self.inherent_impls.clone() }
    fn trait_impls(&self) -> CloneLazy<Vec<TraitImplPath>> { self.trait_impls.clone() }
}
impl SubModuleConstDynMetadata for SubModuleMetadata {
    fn id_path(&self) -> CloneLazy<SubModulePath> { self.id_path.clone() }
    fn sub_modules(&self) -> CloneLazy<Vec<SubModulePath>> { self.sub_modules.clone() }
    fn type_proxy_modules(&self) -> CloneLazy<Vec<TypeProxyModulePath>> { self.type_proxy_modules.clone() }
    fn module_associated_functions(&self) -> CloneLazy<Vec<ModuleAssociatedFunctionPath>> { self.module_associated_functions.clone() }
}
impl SubModuleMetadata {
    pub(super) fn register_sub_module(&self, engine: &mut rhai::Engine, parent_module: &mut rhai::Module) {
        let registry = RAW_REFLECTION_METADATA();
        let mut origin_sub_module = rhai::Module::new();
        origin_sub_module.set_id(self.id_path().get().module_name());

        for path in self.sub_modules().get().into_iter() {
            let sub_module = registry.sub_modules.get(&path).unwrap().clone();
            sub_module.register_sub_module(engine, &mut origin_sub_module);
        }

        for path in self.traits().get().into_iter() {
            let (trait_, trait_object) = registry.traits.get(&path).unwrap().clone();
            trait_.register_trait(&mut origin_sub_module);
            trait_object.register_trait_object(parent_module);
        }

        for path in self.types().get().into_iter() {
            let type_ = registry.types.get(&path).unwrap().clone();
            type_.register_type(engine, &mut origin_sub_module);
        }

        for path in self.type_proxy_modules().get().into_iter() {
            let type_proxy_module = registry.type_proxy_modules.get(&path).unwrap().clone();
            type_proxy_module.register_type_proxy_module(&mut origin_sub_module);
        }

        for path in self.module_associated_functions().get().into_iter() {
            let module_associated_function = registry.module_associated_functions.get(&path).unwrap().clone();
            module_associated_function.register_module_associated_function(&mut origin_sub_module);
        }

        parent_module.set_sub_module(self.id_path().get().module_name(), origin_sub_module);
    }
}
#[derive(Clone)]
pub struct TypeProxyModuleMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<TypeProxyModulePath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,

    pub item_associated_functions: CloneLazy<Vec<ItemAssociatedFunctionPath>>,
    pub constructor_functions: CloneLazy<Vec<ConstructorFunctionPath>>,
}
impl ConstDynMetadata for TypeProxyModuleMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl TypeProxyModuleConstDynMetadata for TypeProxyModuleMetadata {
    fn id_path(&self) -> CloneLazy<TypeProxyModulePath> { self.id_path.clone() }
    fn item_associated_functions(&self) -> CloneLazy<Vec<ItemAssociatedFunctionPath>> { self.item_associated_functions.clone() }
    fn constructor_functions(&self) -> CloneLazy<Vec<ConstructorFunctionPath>> { self.constructor_functions.clone() }
}
impl TypeProxyModuleMetadata {
    fn register_type_proxy_module(&self, parent_module: &mut rhai::Module) {
        let registry = RAW_REFLECTION_METADATA();
        let mut type_proxy_module = rhai::Module::new();
        type_proxy_module.set_id(self.id_path().type_name());

        for path in self.item_associated_functions().get().into_iter() {
            let item_associated_function = registry.item_associated_functions.get(&path).unwrap().clone();
            item_associated_function.register_item_associated_function(&mut type_proxy_module);
        }

        for path in self.constructor_functions().get().into_iter() {
            let constructor_function = registry.constructor_functions.get(&path).unwrap().clone();
            constructor_function.register_constructor_function(&mut type_proxy_module);
        }

        parent_module.set_sub_module(self.id_path().type_name(), type_proxy_module);
    }
}

// Trait Metadata
#[derive(Clone)]
pub struct TraitMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<TraitPath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,
    pub trait_name: &'static str,
}
impl ConstDynMetadata for TraitMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl DynGetTraitName for TraitMetadata {
    fn trait_name(&self) -> &'static str { self.trait_name.clone() }
}
impl TraitConstDynMetadata for TraitMetadata {
    fn id_path(&self) -> CloneLazy<TraitPath> { self.id_path.clone() }
}
impl TraitMetadata {
    pub(super) fn register_trait(&self, parent_module: &mut rhai::Module) where Self: Sized {
        parent_module.set_custom_type::<Self>(self.trait_name());
    }
}
#[derive(Clone)]
pub struct TraitObjectMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<TraitPath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,
    pub trait_object_name: &'static str,
}
impl ConstDynMetadata for TraitObjectMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl DynGetTraitObjectName for TraitObjectMetadata {
    fn trait_object_name(&self) -> &'static str { self.trait_object_name.clone() }
}
impl TraitObjectConstDynMetadata for TraitObjectMetadata {
    fn id_path(&self) -> CloneLazy<TraitPath> { self.id_path.clone() }
}
impl TraitObjectMetadata {
    pub(super) fn register_trait_object(&self, parent_module: &mut rhai::Module) where Self: Sized {
        parent_module.set_custom_type::<Self>(self.trait_object_name());
    }
}

// Type Metadata
#[derive(Clone)]
pub struct TypeMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<TypePath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,

    pub method_functions: CloneLazy<Vec<MethodFunctionPath>>,
}
impl ConstDynMetadata for TypeMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl TypeConstDynMetadata for TypeMetadata {
    fn id_path(&self) -> CloneLazy<TypePath> { self.id_path.clone() }
    fn method_functions(&self) -> CloneLazy<Vec<MethodFunctionPath>> { self.method_functions.clone() }
}
impl TypeMetadata {
    pub(super) fn register_type(&self, engine: &mut rhai::Engine, parent_module: &mut rhai::Module) {
        // TODO: IMPORTANT: This shit is very sketchy and we need to properly handle different types of types
        // TODO: IMPORTANT: and not just blindly wrap every type in a ScopedAccessHandle.
        // TODO: IMPORTANT: Also think about how to auto-register the monomorphized functions and methods and types and whatnot used to properly interact with a type container such as ScopedAccessHandle
        parent_module.set_custom_type::<ScopedAccessHandle<Self>>(self.id_path().type_name());
    }
}

// Impl Metadata
#[derive(Clone)]
pub struct InherentImplMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<InherentImplPath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,

    pub constructor_functions: CloneLazy<Vec<ConstructorFunctionPath>>,
    pub method_functions: CloneLazy<Vec<MethodFunctionPath>>,
}
impl ConstDynMetadata for InherentImplMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl InherentImplConstDynMetadata for InherentImplMetadata {
    fn id_path(&self) -> CloneLazy<InherentImplPath> { self.id_path.clone() }
    fn constructor_functions(&self) -> CloneLazy<Vec<ConstructorFunctionPath>> { self.constructor_functions.clone() }
    fn method_functions(&self) -> CloneLazy<Vec<MethodFunctionPath>> { self.method_functions.clone() }
}
impl InherentImplMetadata {
    pub(super) fn register_inherent_impl(&self, engine: &mut rhai::Engine, type_proxy_module: &mut rhai::Module) {
        let registry = RAW_REFLECTION_METADATA();

        for path in self.constructor_functions().get().into_iter() {
            let constructor_function = registry.constructor_functions.get(&path).unwrap().clone();
            constructor_function.register_constructor_function(type_proxy_module);
        }

        for path in self.method_functions().get().into_iter() {
            let method_function = registry.method_functions.get(&path).unwrap().clone();
            method_function.register_method_function(engine);
        }
    }
}
#[derive(Clone)]
pub struct TraitImplMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<TraitImplPath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,

    pub constructor_functions: CloneLazy<Vec<ConstructorFunctionPath>>,
    pub method_functions: CloneLazy<Vec<MethodFunctionPath>>,
}
impl ConstDynMetadata for TraitImplMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl TraitImplConstDynMetadata for TraitImplMetadata {
    fn id_path(&self) -> CloneLazy<TraitImplPath> { self.id_path.clone() }
    fn constructor_functions(&self) -> CloneLazy<Vec<ConstructorFunctionPath>> { self.constructor_functions.clone() }
    fn method_functions(&self) -> CloneLazy<Vec<MethodFunctionPath>> { self.method_functions.clone() }
}
impl TraitImplMetadata {
    pub(super) fn register_trait_impl(&self, engine: &mut rhai::Engine, type_proxy_module: &mut rhai::Module) {
        let registry = RAW_REFLECTION_METADATA();

        for path in self.constructor_functions().get().into_iter() {
            let constructor_function = registry.constructor_functions.get(&path).unwrap().clone();
            constructor_function.register_constructor_function(type_proxy_module);
        }

        for path in self.method_functions().get().into_iter() {
            let method_function = registry.method_functions.get(&path).unwrap().clone();
            method_function.register_method_function(engine);
        }
    }
}

// Function Metadata
#[derive(Clone)]
pub struct ModuleAssociatedFunctionMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<ModuleAssociatedFunctionPath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,
    pub registrator: CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)>,
}
impl ConstDynMetadata for ModuleAssociatedFunctionMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl ModuleAssociatedFunctionConstDynMetadata for ModuleAssociatedFunctionMetadata {
    fn id_path(&self) -> CloneLazy<ModuleAssociatedFunctionPath> { self.id_path.clone() }
    fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)> { self.registrator }
}
impl ModuleAssociatedFunctionMetadata {
    pub(super) fn register_module_associated_function(mut self, parent_module: &mut rhai::Module) {
        let parent_module = unsafe { std::mem::transmute::<&mut rhai::Module, &'static mut rhai::Module>(parent_module) };
        self.registrator.call_(parent_module);
    }
}
#[derive(Clone)]
pub struct ItemAssociatedFunctionMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<ItemAssociatedFunctionPath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,
    pub registrator: CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)>,
}
impl ConstDynMetadata for ItemAssociatedFunctionMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl ItemAssociatedFunctionConstDynMetadata for ItemAssociatedFunctionMetadata {
    fn id_path(&self) -> CloneLazy<ItemAssociatedFunctionPath> { self.id_path.clone() }
    fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)> { self.registrator }
}
impl ItemAssociatedFunctionMetadata {
    pub(super) fn register_item_associated_function(mut self, type_proxy_module: &mut rhai::Module) {
        let type_proxy_module = unsafe { std::mem::transmute::<&mut rhai::Module, &'static mut rhai::Module>(type_proxy_module) };
        self.registrator.call_(type_proxy_module);
    }
}
#[derive(Clone)]
pub struct ConstructorFunctionMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<ConstructorFunctionPath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,
    pub registrator: CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)>,
}
impl ConstDynMetadata for ConstructorFunctionMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl ConstructorFunctionConstDynMetadata for ConstructorFunctionMetadata {
    fn id_path(&self) -> CloneLazy<ConstructorFunctionPath> { self.id_path.clone() }
    fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)> { self.registrator }
}
impl ConstructorFunctionMetadata {
    pub(super) fn register_constructor_function(mut self, parent_module: &mut rhai::Module) {
        let parent_module = unsafe { std::mem::transmute::<&mut rhai::Module, &'static mut rhai::Module>(parent_module) };
        self.registrator.call_(parent_module);
    }
}
#[derive(Clone)]
pub struct MethodFunctionMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<MethodFunctionPath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,
    pub registrator: CloneClosure<ImmutableString, &'static mut rhai::Engine, (), fn(ImmutableString, &mut rhai::Engine)>,
}
impl ConstDynMetadata for MethodFunctionMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl MethodFunctionConstDynMetadata for MethodFunctionMetadata {
    fn id_path(&self) -> CloneLazy<MethodFunctionPath> { self.id_path.clone() }
    fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Engine, (), fn(ImmutableString, &mut rhai::Engine)> { self.registrator }
}
impl MethodFunctionMetadata {
    pub(super) fn register_method_function(mut self, engine: &mut rhai::Engine) {
        let engine = unsafe { std::mem::transmute::<&mut rhai::Engine, &'static mut rhai::Engine>(engine) };
        self.registrator.call_(engine);
    }
}



// Abstract primitives
pub const trait ConstDynMetadata: 'static + Clone + Send + Sync {
    fn raw_rust_module_path(&self) -> &'static str;
}
pub const trait NativeModuleConstDynMetadata: ConstDynMetadata {
    fn traits(&self) -> CloneLazy<Vec<TraitPath>>;
    fn types(&self) -> CloneLazy<Vec<TypePath>>;
    fn inherent_impls(&self) -> CloneLazy<Vec<InherentImplPath>>;
    fn trait_impls(&self) -> CloneLazy<Vec<TraitImplPath>>;
}

// Modules
pub const trait TopLevelModuleConstDynMetadata: NativeModuleConstDynMetadata {
    fn id_path(&self) -> CloneLazy<TopLevelModulePath>;
    fn sub_modules(&self) -> CloneLazy<Vec<SubModulePath>>;
    fn type_proxy_modules(&self) -> CloneLazy<Vec<TypeProxyModulePath>>;
    fn module_associated_functions(&self) -> CloneLazy<Vec<ModuleAssociatedFunctionPath>>;
}
pub const trait SubModuleConstDynMetadata: NativeModuleConstDynMetadata {
    fn id_path(&self) -> CloneLazy<SubModulePath>;
    fn sub_modules(&self) -> CloneLazy<Vec<SubModulePath>>;
    fn type_proxy_modules(&self) -> CloneLazy<Vec<TypeProxyModulePath>>;
    fn module_associated_functions(&self) -> CloneLazy<Vec<ModuleAssociatedFunctionPath>>;
}
pub const trait TypeProxyModuleConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> CloneLazy<TypeProxyModulePath>;
    fn item_associated_functions(&self) -> CloneLazy<Vec<ItemAssociatedFunctionPath>>;
    fn constructor_functions(&self) -> CloneLazy<Vec<ConstructorFunctionPath>>;
}

// Traits
pub const trait TraitConstDynMetadata: ConstDynMetadata + DynGetTraitName {
    fn id_path(&self) -> CloneLazy<TraitPath>;
}
pub const trait TraitObjectConstDynMetadata: ConstDynMetadata + DynGetTraitObjectName {
    fn id_path(&self) -> CloneLazy<TraitPath>;
}

// Types
/// I think this is outdated, and the entire Type shit is not yet adapted to the new reflection paradigm,
/// AKA there is no metadata to describe the different possible variants of a Type yet
pub const trait TypeConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> CloneLazy<TypePath>;
    fn method_functions(&self) -> CloneLazy<Vec<MethodFunctionPath>>;
}
// pub const trait TypeOwnConstDynMetadata: TypeConstDynMetadata {}
// pub const trait TypeCloneConstDynMetadata: TypeConstDynMetadata {}
// pub const trait TypePersistentRefConstDynMetadata: TypeConstDynMetadata {}
// pub const trait TypePersistentMutConstDynMetadata: TypeConstDynMetadata {}
// /// Like a PersistentRef, but backs a rust-native immutable borrow *with* lifetimes, aka it implements runtime-checks against use-after-free's and aliasing issues; 
// pub const trait TypeScopedRefConstDynMetadata: TypeConstDynMetadata {}
// pub const trait TypeScopedMutConstDynMetadata: TypeConstDynMetadata {}

// Impls
pub const trait InherentImplConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> CloneLazy<InherentImplPath>;
    fn constructor_functions(&self) -> CloneLazy<Vec<ConstructorFunctionPath>>;
    fn method_functions(&self) -> CloneLazy<Vec<MethodFunctionPath>>;
}
pub const trait TraitImplConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> CloneLazy<TraitImplPath>;
    fn constructor_functions(&self) -> CloneLazy<Vec<ConstructorFunctionPath>>;
    fn method_functions(&self) -> CloneLazy<Vec<MethodFunctionPath>>;
}

// Functions
pub const trait ModuleAssociatedFunctionConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> CloneLazy<ModuleAssociatedFunctionPath>;
    fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)>;
}
pub const trait ItemAssociatedFunctionConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> CloneLazy<ItemAssociatedFunctionPath>;
    fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)>;
}
pub const trait ConstructorFunctionConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> CloneLazy<ConstructorFunctionPath>;
    fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)>;
}
pub const trait MethodFunctionConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> CloneLazy<MethodFunctionPath>;
    fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Engine, (), fn(ImmutableString, &mut rhai::Engine)>;
}



// Modules
pub trait TopLevelModuleDynamicTypedMetadata: TopLevelModuleConstDynMetadata {
    fn from_comptime_to_runtime<T: TopLevelModuleConstDynMetadata>(&self, const_dyn_metadata: &T) -> TopLevelModuleMetadata {
        TopLevelModuleMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path().clone(),

            traits: const_dyn_metadata.traits().clone(),
            types: const_dyn_metadata.types().clone(),
            inherent_impls: const_dyn_metadata.inherent_impls().clone(),
            trait_impls: const_dyn_metadata.trait_impls().clone(),

            sub_modules: const_dyn_metadata.sub_modules().clone(),
            type_proxy_modules: const_dyn_metadata.type_proxy_modules().clone(),
            module_associated_functions: const_dyn_metadata.module_associated_functions().clone(),
        }
    }
}
pub trait SubModuleDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: SubModuleConstDynMetadata>(&self, const_dyn_metadata: &T) -> SubModuleMetadata {
        SubModuleMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path(),

            traits: const_dyn_metadata.traits().clone(),
            types: const_dyn_metadata.types().clone(),
            inherent_impls: const_dyn_metadata.inherent_impls().clone(),
            trait_impls: const_dyn_metadata.trait_impls().clone(),

            sub_modules: const_dyn_metadata.sub_modules().clone(),
            type_proxy_modules: const_dyn_metadata.type_proxy_modules().clone(),
            module_associated_functions: const_dyn_metadata.module_associated_functions().clone(),
        }
    }
}
pub trait TypeProxyModuleDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: TypeProxyModuleConstDynMetadata>(&self, const_dyn_metadata: &T) -> TypeProxyModuleMetadata {
        TypeProxyModuleMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path().clone(),

            item_associated_functions: const_dyn_metadata.item_associated_functions().clone(),
            constructor_functions: const_dyn_metadata.constructor_functions().clone(),
        }
    }
}

// Traits
pub trait TraitDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: TraitConstDynMetadata>(&self, const_dyn_metadata: &T) -> TraitMetadata {
        TraitMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path().clone(),
            trait_name: const_dyn_metadata.trait_name(),
        }
    }
}
pub trait TraitObjectDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: TraitObjectConstDynMetadata>(&self, const_dyn_metadata: &T) -> TraitObjectMetadata {
        TraitObjectMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path().clone(),
            trait_object_name: const_dyn_metadata.trait_object_name(),
        }
    }
}
// Types
pub trait TypeDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: TypeConstDynMetadata>(&self, const_dyn_metadata: &T) -> TypeMetadata {
        TypeMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path().clone(),
            
            method_functions: const_dyn_metadata.method_functions().clone(),
        }
    }
}

// Impls
pub trait InherentImplDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: InherentImplConstDynMetadata>(&self, const_dyn_metadata: &T) -> InherentImplMetadata {
        InherentImplMetadata {
            id_path: const_dyn_metadata.id_path(),
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            
            constructor_functions: const_dyn_metadata.constructor_functions().clone(),
            method_functions: const_dyn_metadata.method_functions().clone(),
        }
    }
}
pub trait TraitImplDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: TraitImplConstDynMetadata>(&self, const_dyn_metadata: &T) -> TraitImplMetadata {
        TraitImplMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path(),
            
            constructor_functions: const_dyn_metadata.constructor_functions().clone(),
            method_functions: const_dyn_metadata.method_functions().clone(),
        }
    }
}

// Functions
pub trait ModuleAssociatedFunctionDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: ModuleAssociatedFunctionConstDynMetadata>(&self, const_dyn_metadata: &T) -> ModuleAssociatedFunctionMetadata {
        ModuleAssociatedFunctionMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path(),
            registrator: const_dyn_metadata.clone().registrator(),
        }
    }
}
pub trait ItemAssociatedFunctionDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: ItemAssociatedFunctionConstDynMetadata>(&self, const_dyn_metadata: &T) -> ItemAssociatedFunctionMetadata {
        ItemAssociatedFunctionMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path(),
            registrator: const_dyn_metadata.clone().registrator(),
        }
    }
}
pub trait ConstructorFunctionDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: ConstructorFunctionConstDynMetadata>(&self, const_dyn_metadata: &T) -> ConstructorFunctionMetadata {
        ConstructorFunctionMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path(),
            registrator: const_dyn_metadata.clone().registrator(),
        }
    }
}
pub trait MethodFunctionDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: MethodFunctionConstDynMetadata>(&self, const_dyn_metadata: &T) -> MethodFunctionMetadata {
        MethodFunctionMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path(),
            registrator: const_dyn_metadata.clone().registrator(),
        }
    }
}









// Outdated and old shit that might still be useful below:

pub trait EngineExt {
    fn enable_type_binding(&mut self, fully_qualified_type_path: impl Into<TypeId>) -> &mut Self;
}

impl EngineExt for rhai::Engine {
    fn enable_type_binding(&mut self, fully_qualified_type_path: impl Into<TypeId>) -> &mut Self {
        fn format_function_name(type_id: &TypeId, func_name: &impl std::fmt::Display) -> String {
            format!(
                "{}_{}",
                type_id.to_string().replace("::", "_"),
                func_name
            )
        }

        let type_id: TypeId = fully_qualified_type_path.into();

        let Some(type_info) = TYPE_REGISTRY().get(&type_id) else {
            panic!("Type '{}' not found in TYPE_REGISTRY", type_id);
        };

        // -- Register constructors --
        for ctor_id in &type_info.ctor_ids {
            let ctor_name = format_function_name(&type_id, &ctor_id.sig.name);

            if let Some(&ctor_fn) = CTOR_REGISTRY().get(ctor_id) {
                self.register_fn(ctor_name, ctor_fn);
            } else {
                panic!("Constructor '{}' not found in CTOR_REGISTRY", ctor_id.sig);
            }
        }

        // -- Register methods --
        for method_id in &type_info.method_ids {
            let method_name = format_function_name(&type_id, &method_id.sig.name);

            if let Some(&method_fn) = METHOD_REGISTRY().get(method_id) {
                self.register_fn(method_name, method_fn);
            } else {
                panic!("Method '{}' not found in METHOD_REGISTRY", method_id.sig);
            }
        }

        // -- Register static functions --
        for static_fn_id in &type_info.static_function_ids {
            let static_fn_name = format_function_name(&type_id, &static_fn_id.sig.name);

            if let Some(&static_fn) = STATIC_FUNCTION_REGISTRY().get(static_fn_id) {
                self.register_fn(static_fn_name, static_fn);
            } else {
                panic!("Static '{}' function not found in STATIC_FUNCTION_REGISTRY", static_fn_id.sig);
            }
        }

        self
    }
}


// WIP/SCRATCHPAD \/ \/ \/ \/





use rhai::Dynamic;

/// LinkedMetadata provider for reflection + scripting
pub trait ReflectType {
    fn type_info() -> TypeInfo;
}

/// Constructs Self dynamically
pub trait Constructible: Sized {
    fn construct(ctor: &str, args: Vec<Dynamic>) -> Result<Self, String>;
}

/// Assignable field-wise (for struct composition)
pub trait FieldAssignable {
    fn set_field(&mut self, field: &str, value: Dynamic) -> Result<(), String>;
}















// impl_reflect_type!(f32, ["core"], "f32");
// 
// impl ReflectType for Vec3 {
//     fn type_info() -> TypeInfo {
//         TypeInfo {
//             type_id: TypeId {
//                 module_path: vec!["bevy", "prelude"],
//                 type_name: "Vec3",
//             },
//             type_shape: TypeShape {
//                 kind: TypeKind::Struct,
//                 inner: TypeDataLayout::Struct {
//                     field_infos: vec![
//                         FieldInfo {
//                             name: "x",
//                             type_id: TypeId {
//                                 module_path: vec!["core"],
//                                 type_name: "f32",
//                             },
//                         },
//                         FieldInfo {
//                             name: "y",
//                             type_id: TypeId {
//                                 module_path: vec!["core"],
//                                 type_name: "f32",
//                             },
//                         },
//                         FieldInfo {
//                             name: "z",
//                             type_id: TypeId {
//                                 module_path: vec!["core"],
//                                 type_name: "f32",
//                             },
//                         },
//                     ],
//                 },
//             },
//             ctor_infos: vec![
//                 CtorInfo {
//                     name: "new",
//                     arg_infos: vec![
//                         ArgInfo {
//                             name: "x",
//                             type_id: TypeId {
//                                 module_path: vec!["core"],
//                                 type_name: "f32",
//                             },
//                         },
//                         ArgInfo {
//                             name: "y",
//                             type_id: TypeId {
//                                 module_path: vec!["core"],
//                                 type_name: "f32",
//                             },
//                         },
//                         ArgInfo {
//                             name: "z",
//                             type_id: TypeId {
//                                 module_path: vec!["core"],
//                                 type_name: "f32",
//                             },
//                         },
//                     ],
//                 },
//                 CtorInfo {
//                     name: "default",
//                     arg_infos: vec![],
//                 },
//             ],
//             method_infos: vec![],
//         }
//     }
// }
// 
// impl Constructible for Vec3 {
//     fn construct(ctor: &str, args: Vec<Dynamic>) -> Result<Self, String> {
//         match ctor {
//             "default" if args.len() == 0 => Ok(Vec3::default()),
//             "new" if args.len() == 3 => Ok(Vec3 {
//                 x: args[0].clone_cast::<f32>(),
//                 y: args[1].clone_cast::<f32>(),
//                 z: args[2].clone_cast::<f32>(),
//             }),
//             _ => Err("Invalid ctor".into()),
//         }
//     }
// }











// pub(crate) trait Composable: Sized {
//     fn composition_info() -> CompositionInfo;
//     fn construct(method: &str, args: Box<dyn Any>) -> Result<Self, &str>;
//     fn modify(&mut self, method: &str, args: Box<dyn Any>) -> Result<(), &str>;
// }
// 
// use crate::bevy::prelude::{Transform, Vec3, Quat, Color};
// use wherever::{MovementBundle, whatever};
// 
// extern_composable!(
//     extern_type: Transform,
//     location: "crate::bevy::transform::components"
//     composition_type: Component,
//     fields: [
//         "translation": Vec3,
//         "rotation": Quat,
//         "scale": Vec3,
//     ],
//     ctors: [
//         ["default": Default::default()],
//         ["from_translation": Self::from_translation(translation: Vec3)],
//         ["from_rotation": Self::from_rotation(rotation: Quat)],
//         ["from_scale": Self::from_scale(scale: Vec3)],
//     ],
// );
// 
// #[self_composable(
//     location: "enemy::bundles"
//     composition_type: Bundle,   // Requires Default
//     fields: [
//         Component("transform": Transform),
//         Component("sprite": crate::bevy::prelude::Sprite),
//         Bundle("movement_bundle": MovementBundle),
//         Component("enemy_ai": whatever::EnemyAi),
//     ],
//     ctors: [
//         ["default": Default::default()],
//         ["new_orc": Self::new_orc(color: Color)],
//     ],
// )]
// pub struct EnemyBundle {
//     transform: Transform,   // impls Default
//     sprite: crate::bevy::prelude::Sprite,         // impls Default
//     movement_bundle: MovementBundle,
//     enemy_ai: whatever::EnemyAi,      // can not be implicitly defaulted, but does provide a placeholder value
// }
// impl Default for EnemyBundle {
//     fn default() -> Self {
//         EnemyBundle {
//             transform: Default::default(),
//             sprite: Default::default(),
//             movement_bundle: Default::default(),
//             enemy_ai: whatever::EnemyAi::placeholder(),
//         }
//     }
// }
// impl EnemyBundle {
//     pub fn new_orc(color: Color) -> Self {
//         EnemyBundle {
//             transform: Transform::default(),
//             sprite: crate::bevy::prelude::Sprite {
//                 color,
//                 ..Default::default()
//             },
//             movement_bundle: MovementBundle::default(),
//             enemy_ai: whatever::EnemyAi::new("orc"),
//         }
//     }
// }






