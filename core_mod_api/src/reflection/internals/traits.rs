#![allow(clippy::missing_safety_doc)]

use once_cell::sync::Lazy;
use rhai::{RhaiNativeFunc, Shared, Variant};
use std::hash::Hash;
use std::sync::Arc;

use crate::reflection::traits::StaticTraitObject;


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
    use crate::reflection::internals::traits::*;
    use core_mod_macros::reflect_top_level_module;

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
    inventory::submit!(__Shop__TopLevelModule__.build());

    #[allow(non_camel_case_types)]
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct __Shop__TopLevelModule__;
    impl ConstDynMetadata for __Shop__TopLevelModule__ {
        fn raw_rust_module_path(&self) -> &'static str { module_path!() }
    }
    impl NativeModuleConstDynMetadata for __Shop__TopLevelModule__ {
        fn traits(&self) -> Lazy<Vec<TraitPath>> {
            Lazy::new(|| vec![])
        }
        fn types(&self) -> Lazy<Vec<TypePath>> {
            Lazy::new(|| vec![])
        }
        fn inherent_impls(&self) -> Lazy<Vec<InherentImplPath>> {
            Lazy::new(|| vec![])
        }
        fn trait_impls(&self) -> Lazy<Vec<TraitImplPath>> {
            Lazy::new(|| vec![])
        }
    }
    impl TopLevelModuleConstDynMetadata for __Shop__TopLevelModule__ {
        fn id_path(&self) -> TopLevelModulePath {
            "shop".into()
        }
        fn sub_modules(&self) -> Vec<SubModulePath> {
            vec!["shop::divisions".into()]
        }
        fn type_proxy_modules(&self) -> Vec<TypeProxyModulePath> {
            vec![]
        }
        fn module_associated_functions(&self) -> Vec<ModuleAssociatedFunctionPath> {
            vec![]
        }
    }
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> //

// The actual end-user code
// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< //
    pub mod divisions {
        use crate::reflection::internals::traits::*;
        use core_mod_macros::reflect_sub_module;

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
        inventory::submit!(__Divisions__SubModule__.build());

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub struct __Divisions__SubModule__;
        impl ConstDynMetadata for __Divisions__SubModule__ {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }
        impl NativeModuleConstDynMetadata for __Divisions__SubModule__ {
            fn traits(&self) -> Lazy<Vec<TraitPath>> {
                Lazy::new(|| vec![])
            }
            fn types(&self) -> Lazy<Vec<TypePath>> {
                Lazy::new(|| vec![])
            }
            fn inherent_impls(&self) -> Lazy<Vec<InherentImplPath>> {
                Lazy::new(|| vec![])
            }
            fn trait_impls(&self) -> Lazy<Vec<TraitImplPath>> {
                Lazy::new(|| vec![])
            }
        }
        impl SubModuleConstDynMetadata for __Divisions__SubModule__ {
            fn id_path(&self) -> SubModulePath {
                "shop::divisions".into()
            }
            fn sub_modules(&self) -> Vec<SubModulePath> {
                vec!["shop::divisions::sex".into()]
            }
            fn type_proxy_modules(&self) -> Vec<TypeProxyModulePath> {
                vec![]
            }
            fn module_associated_functions(&self) -> Vec<ModuleAssociatedFunctionPath> {
                vec![]
            }
        }
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> //

// The actual end-user code
// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< //
        pub mod sex {
            use crate::{reflection::internals::traits::*, utils::string::MethodFunctionPath};
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
            inventory::submit!(__Sex__SubModule__.from_comptime_to_runtime());
            inventory::submit!(__SexShopTest__Trait__.from_comptime_to_runtime());
            inventory::submit!(__SexShopTest__TraitObject__.from_comptime_to_runtime());
            inventory::submit!(__SexShopProduct__Type__.from_comptime_to_runtime());
            inventory::submit!(__TestFunction__ModuleAssociatedFunction__.from_comptime_to_runtime());
            inventory::submit!(__VerifyPrice__ItemAssociatedFunction__.from_comptime_to_runtime());
            inventory::submit!(__New__ConstructorFunction__.from_comptime_to_runtime());
            inventory::submit!(__Name__MethodFunction__.from_comptime_to_runtime());
            inventory::submit!(__PriceUsd__MethodFunction__.from_comptime_to_runtime());

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __Sex__SubModule__;
            impl ConstDynMetadata for __Sex__SubModule__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl NativeModuleConstDynMetadata for __Sex__SubModule__ {
                fn traits(&self) -> Lazy<Vec<TraitPath>> { Lazy::new(|| vec!["shop::divisions::sex::SexShopTest".into()]) }
                fn types(&self) -> Lazy<Vec<TypePath>> { Lazy::new(|| vec!["shop::divisions::sex::SexShopProduct".into()]) }
                fn inherent_impls(&self) -> Lazy<Vec<InherentImplPath>> { Lazy::new(|| vec![]) }
                fn trait_impls(&self) -> Lazy<Vec<TraitImplPath>> { Lazy::new(|| vec![]) }
            }
            impl SubModuleConstDynMetadata for __Sex__SubModule__ {
                fn id_path(&self) -> SubModulePath { "shop::divisions::sex".into() }
                fn sub_modules(&self) -> Vec<SubModulePath> { vec![] }
                fn type_proxy_modules(&self) -> Vec<TypeProxyModulePath> { vec![] }
                fn module_associated_functions(&self) -> Vec<ModuleAssociatedFunctionPath> { vec!["shop::divisions::sex::test_function".into()] }
            }

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
                fn id_path(&self) -> TraitPath { "shop::divisions::sex::SexShopTest".into() }
            }

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
                fn id_path(&self) -> TraitPath { "shop::divisions::sex::SexShopTestTraitObject".into() }
            }

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __SexShopProduct__Type__;
            impl ConstDynMetadata for __SexShopProduct__Type__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl TypeConstDynMetadata for __SexShopProduct__Type__ {
                fn id_path(&self) -> TypePath { "shop::divisions::sex::SexShopProduct".into() }
                fn method_functions(&self) -> Vec<MethodFunctionPath> {
                    vec![
                        "shop::divisions::sex::SexShopProduct::name".into(),
                        "shop::divisions::sex::SexShopProduct::price_usd".into(),
                    ]
                }
            }

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __SexShopProduct__TypeProxyModule__;
            impl ConstDynMetadata for __SexShopProduct__TypeProxyModule__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl NativeModuleConstDynMetadata for __SexShopProduct__TypeProxyModule__ {
                fn traits(&self) -> Lazy<Vec<TraitPath>> { Lazy::new(|| vec![]) }
                fn types(&self) -> Lazy<Vec<TypePath>> { Lazy::new(|| vec![]) }
                fn inherent_impls(&self) -> Lazy<Vec<InherentImplPath>> { Lazy::new(|| vec![]) }
                fn trait_impls(&self) -> Lazy<Vec<TraitImplPath>> { Lazy::new(|| vec![]) }
            }
            impl TypeProxyModuleConstDynMetadata for __SexShopProduct__TypeProxyModule__ {
                fn id_path(&self) -> TypeProxyModulePath { "shop::divisions::sex::SexShopProduct".into() }
                fn item_associated_functions(&self) -> Vec<ItemAssociatedFunctionPath> {
                    vec!["shop::divisions::sex::SexShopProduct::new".into()]
                }
                fn constructor_functions(&self) -> Vec<ConstructorFunctionPath> {
                    vec!["shop::divisions::sex::SexShopProduct::verify_price".into()]
                }
            }
            impl TypeProxyModuleDynamicTypedMetadata for __SexShopProduct__TypeProxyModule__ {

            }

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __TestFunction__ModuleAssociatedFunction__;
            impl ConstDynMetadata for __TestFunction__ModuleAssociatedFunction__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl ModuleAssociatedFunctionConstDynMetadata for __TestFunction__ModuleAssociatedFunction__ {
                fn id_path(&self) -> ModuleAssociatedFunctionPath { "shop::divisions::sex::test_function".into() }
                fn registrator(&self) -> Box<dyn FnOnce(&mut rhai::Module) + Send + Sync> {
                    let name = self.id_path().function_name().clone();
                    let func = move |parent_module: &mut rhai::Module| {
                        rhai::FuncRegistration::new(name)
                            .set_into_module(parent_module, test_function);
                    };
                    Box::new(func)
                }
            }

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __VerifyPrice__ItemAssociatedFunction__;
            impl ConstDynMetadata for __VerifyPrice__ItemAssociatedFunction__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl ItemAssociatedFunctionConstDynMetadata for __VerifyPrice__ItemAssociatedFunction__ {
                fn id_path(&self) -> ItemAssociatedFunctionPath { "shop::divisions::sex::SexShopProduct::verify_price".into() }
                fn registrator(&self) -> Box<dyn FnOnce(&mut rhai::Module) + Send + Sync> {
                    let name = self.id_path().function_name().clone();
                    let func = move |parent_module: &mut rhai::Module| {
                        rhai::FuncRegistration::new(name)
                            .set_into_module(parent_module, SexShopProduct::verify_price);
                    };
                    Box::new(func)
                }
            }

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __New__ConstructorFunction__;
            impl ConstDynMetadata for __New__ConstructorFunction__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl ConstructorFunctionConstDynMetadata for __New__ConstructorFunction__ {
                fn id_path(&self) -> ConstructorFunctionPath { "shop::divisions::sex::SexShopProduct::new".into() }
                fn registrator(&self) -> Box<dyn FnOnce(&mut rhai::Module) + Send + Sync> {
                    let name = self.id_path().function_name().clone();
                    let func = move |parent_module: &mut rhai::Module| {
                        rhai::FuncRegistration::new(name)
                            .set_into_module(parent_module, SexShopProduct::new);
                    };
                    Box::new(func)
                }
            }

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __Name__MethodFunction__;
            impl ConstDynMetadata for __Name__MethodFunction__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl MethodFunctionConstDynMetadata for __Name__MethodFunction__ {
                fn id_path(&self) -> MethodFunctionPath { "shop::divisions::sex::SexShopProduct::name".into() }
                fn registrator(&self) -> Box<dyn FnOnce(&mut rhai::Engine) + Send + Sync> {
                    let name = self.id_path().function_name().clone();
                    let func = move |engine: &mut rhai::Engine| {
                        engine.register_fn(name, SexShopProduct::name);
                    };
                    Box::new(func)
                }
            }
            
            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __PriceUsd__MethodFunction__;
            impl ConstDynMetadata for __PriceUsd__MethodFunction__ {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl MethodFunctionConstDynMetadata for __PriceUsd__MethodFunction__ {
                fn id_path(&self) -> MethodFunctionPath { "shop::divisions::sex::SexShopProduct::price_usd".into() }
                fn registrator(&self) -> Box<dyn FnOnce(&mut rhai::Engine) + Send + Sync> {
                    let name = self.id_path().function_name().clone();
                    let func = move |engine: &mut rhai::Engine| {
                        engine.register_fn(name, SexShopProduct::price_usd);
                    };
                    Box::new(func)
                }
            }
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> //

// The actual end-user code
// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< //
        }
    }
}
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> //



// TODO: IMPORTANT: Copy all the Box<dyn Thing> from LinkedMetadata to Metadata, but like change it so it doesn't link to other pieces of Metadata, 
// so that Metadata can even contain the raw info that should merely be aggragated/assembled via *LinkedMetadata, 
// but currently LinkedMetadata wrongly assumes responsibility for making sure a Thing can actually be registered;
// that should be the job of the Metadata.



// Module Metadata
inventory::collect!(TopLevelModuleMetadata);
#[derive(Clone)]
pub struct TopLevelModuleMetadata {
    /// Primary means of identification
    pub id_path: TopLevelModulePath,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,

    pub traits: Vec<TraitPath>,
    pub types: Vec<TypePath>,
    pub inherent_impls: Vec<InherentImplPath>,
    pub trait_impls: Vec<TraitImplPath>,

    pub sub_modules: Vec<SubModulePath>,
    pub type_proxy_modules: Vec<TypeProxyModulePath>,
    pub module_associated_functions: Vec<ModuleAssociatedFunctionPath>,
}
impl ConstDynMetadata for TopLevelModuleMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl NativeModuleConstDynMetadata for TopLevelModuleMetadata {
    fn traits(&self) -> Lazy<Vec<TraitPath>> { Lazy::new(|| self.traits.clone()) }
    fn types(&self) -> Lazy<Vec<TypePath>> { Lazy::new(|| self.types.clone()) }
    fn inherent_impls(&self) -> Lazy<Vec<InherentImplPath>> { Lazy::new(|| self.inherent_impls.clone()) }
    fn trait_impls(&self) -> Lazy<Vec<TraitImplPath>> { Lazy::new(|| self.trait_impls.clone()) }
}
impl TopLevelModuleConstDynMetadata for TopLevelModuleMetadata {
    fn id_path(&self) -> TopLevelModulePath { self.id_path.clone() }
    fn sub_modules(&self) -> Lazy<Vec<SubModulePath>> { Lazy::new(|| self.sub_modules.clone()) }
    fn type_proxy_modules(&self) -> Lazy<Vec<TypeProxyModulePath>> { Lazy::new(|| self.type_proxy_modules.clone()) }
    fn module_associated_functions(&self) -> Lazy<Vec<ModuleAssociatedFunctionPath>> { Lazy::new(|| self.module_associated_functions.clone()) }
}
inventory::collect!(SubModuleMetadata);
#[derive(Clone)]
pub struct SubModuleMetadata {
    /// Primary means of identification
    pub id_path: SubModulePath,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,

    pub traits: Vec<TraitPath>,
    pub types: Vec<TypePath>,
    pub inherent_impls: Vec<InherentImplPath>,
    pub trait_impls: Vec<TraitImplPath>,

    pub sub_modules: Vec<SubModulePath>,
    pub type_proxy_modules: Vec<TypeProxyModulePath>,
    pub module_associated_functions: Vec<ModuleAssociatedFunctionPath>,
}
impl ConstDynMetadata for SubModuleMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl NativeModuleConstDynMetadata for SubModuleMetadata {
    fn traits(&self) -> Lazy<Vec<TraitPath>> { Lazy::new(|| self.traits.clone()) }
    fn types(&self) -> Lazy<Vec<TypePath>> { Lazy::new(|| self.types.clone()) }
    fn inherent_impls(&self) -> Lazy<Vec<InherentImplPath>> { Lazy::new(|| self.inherent_impls.clone()) }
    fn trait_impls(&self) -> Lazy<Vec<TraitImplPath>> { Lazy::new(|| self.trait_impls.clone()) }
}
impl SubModuleConstDynMetadata for SubModuleMetadata {
    fn id_path(&self) -> SubModulePath { self.id_path.clone() }
    fn sub_modules(&self) -> Lazy<Vec<SubModulePath>> { Lazy::new(|| self.sub_modules.clone()) }
    fn type_proxy_modules(&self) -> Lazy<Vec<TypeProxyModulePath>> { Lazy::new(|| self.type_proxy_modules.clone()) }
    fn module_associated_functions(&self) -> Lazy<Vec<ModuleAssociatedFunctionPath>> { Lazy::new(|| self.module_associated_functions.clone()) }
}
inventory::collect!(TypeProxyModuleMetadata);
#[derive(Clone)]
pub struct TypeProxyModuleMetadata {
    /// Primary means of identification
    pub id_path: TypeProxyModulePath,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,

    pub item_associated_functions: Vec<ItemAssociatedFunctionPath>,
    pub constructor_functions: Vec<ConstructorFunctionPath>,
}
impl ConstDynMetadata for TypeProxyModuleMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl TypeProxyModuleConstDynMetadata for TypeProxyModuleMetadata {
    fn id_path(&self) -> TypeProxyModulePath { self.id_path.clone() }
    fn item_associated_functions(&self) -> Lazy<Vec<ItemAssociatedFunctionPath>> { Lazy::new(|| self.item_associated_functions.clone()) }
    fn constructor_functions(&self) -> Lazy<Vec<ConstructorFunctionPath>> { Lazy::new(|| self.constructor_functions.clone()) }
}

// Trait Metadata
inventory::collect!(TraitMetadata);
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TraitMetadata {
    /// Primary means of identification
    pub id_path: TraitPath,
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
    fn id_path(&self) -> TraitPath { self.id_path.clone() }
}
inventory::collect!(TraitObjectMetadata);
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TraitObjectMetadata {
    /// Primary means of identification
    pub id_path: TraitPath,
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
    fn id_path(&self) -> TraitPath { self.id_path.clone() }
}

// Type Metadata
inventory::collect!(TypeMetadata);
#[derive(Clone)]
pub struct TypeMetadata {
    /// Primary means of identification
    pub id_path: TypePath,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,

    pub method_functions: Vec<MethodFunctionPath>,
}
impl ConstDynMetadata for TypeMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl TypeConstDynMetadata for TypeMetadata {
    fn id_path(&self) -> TypePath { self.id_path.clone() }
    fn method_functions(&self) -> Lazy<Vec<MethodFunctionPath>> { Lazy::new(|| self.method_functions.clone()) }
}

// Impl Metadata
inventory::collect!(InherentImplMetadata);
#[derive(Clone)]
pub struct InherentImplMetadata {
    /// Primary means of identification
    pub id_path: InherentImplPath,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,

    pub constructor_functions: Vec<ConstructorFunctionPath>,
    pub method_functions: Vec<MethodFunctionPath>,
}
impl ConstDynMetadata for InherentImplMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl InherentImplConstDynMetadata for InherentImplMetadata {
    fn id_path(&self) -> InherentImplPath { self.id_path.clone() }
    fn constructor_functions(&self) -> Lazy<Vec<ConstructorFunctionPath>> { Lazy::new(|| self.constructor_functions.clone()) }
    fn method_functions(&self) -> Lazy<Vec<MethodFunctionPath>> { Lazy::new(|| self.method_functions.clone()) }
}
inventory::collect!(TraitImplMetadata);
#[derive(Clone)]
pub struct TraitImplMetadata {
    /// Primary means of identification
    pub id_path: TraitImplPath,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,

    pub constructor_functions: Vec<ConstructorFunctionPath>,
    pub method_functions: Vec<MethodFunctionPath>,
}
impl ConstDynMetadata for TraitImplMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl TraitImplConstDynMetadata for TraitImplMetadata {
    fn id_path(&self) -> TraitImplPath { self.id_path.clone() }
    fn constructor_functions(&self) -> Lazy<Vec<ConstructorFunctionPath>> { Lazy::new(|| self.constructor_functions.clone()) }
    fn method_functions(&self) -> Lazy<Vec<MethodFunctionPath>> { Lazy::new(|| self.method_functions.clone()) }
}

// Function Metadata
inventory::collect!(ModuleAssociatedFunctionMetadata);
#[derive(Clone)]
pub struct ModuleAssociatedFunctionMetadata {
    /// Primary means of identification
    pub id_path: ModuleAssociatedFunctionPath,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,
    /// Manual registration, which is required for functions,
    /// because rhai does not fully expose the items involved in the trait bounds that define a rhai function.
    pub registrator: Box<dyn FnOnce(&mut rhai::Module) + Send + Sync>,
}
impl ConstDynMetadata for ModuleAssociatedFunctionMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl ModuleAssociatedFunctionConstDynMetadata for ModuleAssociatedFunctionMetadata {
    fn id_path(&self) -> ModuleAssociatedFunctionPath { self.id_path.clone() }
    fn registrator(&self) -> Box<dyn FnOnce(&mut rhai::Module) + Send + Sync> { self.registrator.clone() }
}
inventory::collect!(ItemAssociatedFunctionMetadata);
#[derive(Clone)]
pub struct ItemAssociatedFunctionMetadata {
    /// Primary means of identification
    pub id_path: ItemAssociatedFunctionPath,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,
    /// Manual registration, which is required for functions,
    /// because rhai does not fully expose the items involved in the trait bounds that define a rhai function.
    pub registrator: Box<dyn FnOnce(&mut rhai::Module) + Send + Sync>,
}
impl ConstDynMetadata for ItemAssociatedFunctionMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl ItemAssociatedFunctionConstDynMetadata for ItemAssociatedFunctionMetadata {
    fn id_path(&self) -> ItemAssociatedFunctionPath { self.id_path.clone() }
    fn registrator(&self) -> Box<dyn FnOnce(&mut rhai::Module) + Send + Sync> { self.registrator.clone() }
}
inventory::collect!(ConstructorFunctionMetadata);
#[derive(Clone)]
pub struct ConstructorFunctionMetadata {
    /// Primary means of identification
    pub id_path: ConstructorFunctionPath,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,
    /// Manual registration, which is required for functions,
    /// because rhai does not fully expose the items involved in the trait bounds that define a rhai function.
    pub registrator: Box<dyn FnOnce(&mut rhai::Module) + Send + Sync>,
}
impl ConstDynMetadata for ConstructorFunctionMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl ConstructorFunctionConstDynMetadata for ConstructorFunctionMetadata {
    fn id_path(&self) -> ConstructorFunctionPath { self.id_path.clone() }
    fn registrator(&self) -> Box<dyn FnOnce(&mut rhai::Module) + Send + Sync> { self.registrator.clone() }
}
inventory::collect!(MethodFunctionMetadata);
#[derive(Clone)]
pub struct MethodFunctionMetadata {
    /// Primary means of identification
    pub id_path: MethodFunctionPath,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,
    /// Manual registration, which is required for functions,
    /// because rhai does not fully expose the items involved in the trait bounds that define a rhai function.
    pub registrator: Box<dyn FnOnce(&mut rhai::Engine) + Send + Sync>,
}
impl ConstDynMetadata for MethodFunctionMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path.clone() }
}
impl MethodFunctionConstDynMetadata for MethodFunctionMetadata {
    fn id_path(&self) -> MethodFunctionPath { self.id_path.clone() }
    fn registrator(&self) -> Box<dyn FnOnce(&mut rhai::Engine) + Send + Sync> { self.registrator.clone() }
}

// TODO: IMPORTANT: We just make each Trait below us also have methods to simply get an array/vec/whatever of the `*Path`s to the different sub modules, types, etc.
// TODO: IMPORTANT: We can then default-implement all register_* methods by just getting the different collections, and potentially interpreting each item based on how the respective item's Metadata is structured.

// TODO: IMPORTANT: First step: Finish this preliminarily by properly realizing all registration functionality



// Abstract primitives
pub const trait ConstDynMetadata: 'static + Send + Sync {
    fn raw_rust_module_path(&self) -> &'static str;
}
pub const trait NativeModuleConstDynMetadata: ConstDynMetadata {
    fn traits(&self) -> Lazy<Vec<TraitPath>>;
    fn types(&self) -> Lazy<Vec<TypePath>>;
    fn inherent_impls(&self) -> Lazy<Vec<InherentImplPath>>;
    fn trait_impls(&self) -> Lazy<Vec<TraitImplPath>>;
}

// Modules
pub const trait TopLevelModuleConstDynMetadata: NativeModuleConstDynMetadata {
    fn id_path(&self) -> TopLevelModulePath;
    fn sub_modules(&self) -> Lazy<Vec<SubModulePath>>;
    fn type_proxy_modules(&self) -> Lazy<Vec<TypeProxyModulePath>>;
    fn module_associated_functions(&self) -> Lazy<Vec<ModuleAssociatedFunctionPath>>;

    fn register_top_level_module<T: TopLevelModuleConstDynMetadata>(&self, engine: &mut rhai::Engine) {
        let registry = RAW_REFLECTION_METADATA();
        let mut top_level_module = rhai::Module::new();
        top_level_module.set_id(self.id_path().module_name());

        for path in self.sub_modules().into_iter() {
            let sub_module = registry.sub_modules.get(&path).unwrap();
            sub_module.register_sub_module(engine, &mut top_level_module);
        }

        for path in self.traits().into_iter() {
            let (trait_, trait_object) = registry.traits.get(&path).unwrap();
            trait_.register_trait(&mut top_level_module);
            trait_object.register_trait_object(&mut top_level_module);
        }

        for path in self.types().into_iter() {
            let type_ = registry.types.get(&path).unwrap();
            type_.register_type(engine, &mut top_level_module);
        }

        for path in self.type_proxy_modules().into_iter() {
            let type_proxy_module = registry.type_proxy_modules.get(&path).unwrap();
            type_proxy_module.register_type_proxy_module(&mut top_level_module);
        }

        for path in self.module_associated_functions().into_iter() {
            let module_associated_function = registry.module_associated_functions.get(&path).unwrap();
            module_associated_function.register_module_associated_function(&mut top_level_module);
        }

        engine.register_static_module(self.id_path().module_name(), Arc::new(top_level_module));
    }
}
pub const trait SubModuleConstDynMetadata: NativeModuleConstDynMetadata {
    fn id_path(&self) -> SubModulePath;
    fn sub_modules(&self) -> Lazy<Vec<SubModulePath>>;
    fn type_proxy_modules(&self) -> Lazy<Vec<TypeProxyModulePath>>;
    fn module_associated_functions(&self) -> Lazy<Vec<ModuleAssociatedFunctionPath>>;

    fn register_sub_module<T: SubModuleConstDynMetadata>(&self, engine: &mut rhai::Engine, parent_module: &mut rhai::Module) {
        let registry = RAW_REFLECTION_METADATA();
        let mut origin_sub_module = rhai::Module::new();
        origin_sub_module.set_id(self.id_path().module_name());

        for path in self.sub_modules().into_iter() {
            let sub_module = registry.sub_modules.get(&path).unwrap();
            sub_module.register_sub_module(engine, &mut origin_sub_module);
        }

        for path in self.traits().into_iter() {
            let (trait_, trait_object) = registry.traits.get(&path).unwrap();
            trait_.register_trait(&mut origin_sub_module);
            trait_object.register_trait_object(parent_module);
        }

        for path in self.types().into_iter() {
            let type_ = registry.types.get(&path).unwrap();
            type_.register_type(engine, &mut origin_sub_module);
        }

        for path in self.type_proxy_modules().into_iter() {
            let type_proxy_module = registry.type_proxy_modules.get(&path).unwrap();
            type_proxy_module.register_type_proxy_module(&mut origin_sub_module);
        }

        for path in self.module_associated_functions().into_iter() {
            let module_associated_function = registry.module_associated_functions.get(&path).unwrap();
            module_associated_function.register_module_associated_function(&mut origin_sub_module);
        }

        parent_module.set_sub_module(self.id_path().module_name(), origin_sub_module);
    }
}
pub const trait TypeProxyModuleConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> TypeProxyModulePath;
    fn item_associated_functions(&self) -> Lazy<Vec<ItemAssociatedFunctionPath>>;
    fn constructor_functions(&self) -> Lazy<Vec<ConstructorFunctionPath>>;

    fn register_type_proxy_module<T: TypeProxyModuleConstDynMetadata>(&self, parent_module: &mut rhai::Module) {
        let registry = RAW_REFLECTION_METADATA();
        let mut type_proxy_module = rhai::Module::new();
        type_proxy_module.set_id(self.id_path().type_name());

        for path in self.item_associated_functions().clone().into_iter() {
            let item_associated_function = registry.item_associated_functions.get(&path).unwrap();
            item_associated_function.register_item_associated_function(&mut type_proxy_module);
        }

        for path in self.constructor_functions().clone().into_iter() {
            let constructor_function = registry.constructor_functions.get(&path).unwrap();
            constructor_function.register_constructor_function(&mut type_proxy_module);
        }

        parent_module.set_sub_module(self.id_path().type_name(), type_proxy_module);
    }
}

// Traits
pub const trait TraitConstDynMetadata: ConstDynMetadata + DynGetTraitName {
    fn id_path(&self) -> TraitPath;

    fn register_trait(&self, parent_module: &mut rhai::Module) where Self: Sized {
        parent_module.set_custom_type::<Self>(self.trait_name());
    }
}
pub const trait TraitObjectConstDynMetadata: ConstDynMetadata + DynGetTraitObjectName {
    fn id_path(&self) -> TraitPath;

    fn register_trait_object(&self, parent_module: &mut rhai::Module) where Self: Sized {
        parent_module.set_custom_type::<Self>(self.trait_object_name());
    }
}

// Types
/// I think this is outdated, and the entire Type shit is not yet adapted to the new reflection paradigm,
/// AKA there is no metadata to describe the different possible variants of a Type yet
pub const trait TypeConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> TypePath;
    fn method_functions(&self) -> Lazy<Vec<MethodFunctionPath>>;

    fn register_type(&self, engine: &mut rhai::Engine, parent_module: &mut rhai::Module) {
        parent_module.set_custom_type::<ScopedAccessHandle<Self>>(self.id_path().type_name());
    }
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
    fn id_path(&self) -> InherentImplPath;
    fn constructor_functions(&self) -> Lazy<Vec<ConstructorFunctionPath>>;
    fn method_functions(&self) -> Lazy<Vec<MethodFunctionPath>>;

    fn register_inherent_impl(&self, engine: &mut rhai::Engine, type_proxy_module: &mut rhai::Module) {
        let registry = RAW_REFLECTION_METADATA();

        for path in self.constructor_functions().clone().into_iter() {
            let constructor_function = registry.constructor_functions.get(&path).unwrap();
            constructor_function.register_constructor_function(type_proxy_module);
        }

        for path in self.method_functions().clone().into_iter() {
            let method_function = registry.method_functions.get(&path).unwrap();
            method_function.register_method_function(engine);
        }
    }
}
pub const trait TraitImplConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> TraitImplPath;
    fn constructor_functions(&self) -> Lazy<Vec<ConstructorFunctionPath>>;
    fn method_functions(&self) -> Lazy<Vec<MethodFunctionPath>>;

    fn register_trait_impl(&self, engine: &mut rhai::Engine, type_proxy_module: &mut rhai::Module) {
        let registry = RAW_REFLECTION_METADATA();

        for path in self.constructor_functions().clone().into_iter() {
            let constructor_function = registry.constructor_functions.get(&path).unwrap();
            constructor_function.register_constructor_function(type_proxy_module);
        }

        for path in self.method_functions().clone().into_iter() {
            let method_function = registry.method_functions.get(&path).unwrap();
            method_function.register_method_function(engine);
        }
    }
}

// Functions
pub const trait ModuleAssociatedFunctionConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> ModuleAssociatedFunctionPath;
    fn registrator(&self) -> Box<dyn FnOnce(&mut rhai::Module) + Send + Sync>;
    
    fn register_module_associated_function(&self, parent_module: &mut rhai::Module) {
        (self.registrator())(parent_module);
    }
}
pub const trait ItemAssociatedFunctionConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> ItemAssociatedFunctionPath;
    fn registrator(&self) -> Box<dyn FnOnce(&mut rhai::Module) + Send + Sync>;
    
    fn register_item_associated_function(&self, type_proxy_module: &mut rhai::Module) {
        (self.registrator())(type_proxy_module);
    }
}
pub const trait ConstructorFunctionConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> ConstructorFunctionPath;
    fn registrator(&self) -> Box<dyn FnOnce(&mut rhai::Module) + Send + Sync>;
    
    fn register_constructor_function(&self, parent_module: &mut rhai::Module) {
        (self.registrator())(parent_module);
    }
}
pub const trait MethodFunctionConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> MethodFunctionPath;
    fn registrator(&self) -> Box<dyn FnOnce(&mut rhai::Engine) + Send + Sync>;
    
    fn register_method_function(&self, engine: &mut rhai::Engine) {
        (self.registrator())(engine);
    }
}



// Modules
pub trait TopLevelModuleDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: TopLevelModuleConstDynMetadata>(&self, const_dyn_metadata: &T) -> TopLevelModuleMetadata {
        TopLevelModuleMetadata {
            id_path: const_dyn_metadata.id_path(),
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),

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
            id_path: const_dyn_metadata.id_path(),
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),

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
            id_path: const_dyn_metadata.id_path(),
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),

            item_associated_functions: const_dyn_metadata.item_associated_functions().clone(),
            constructor_functions: const_dyn_metadata.constructor_functions().clone(),
        }
    }
}

// Traits
pub const trait TraitDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: TraitConstDynMetadata>(&self, const_dyn_metadata: &T) -> TraitMetadata {
        TraitMetadata {
            id_path: const_dyn_metadata.id_path(),
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            trait_name: const_dyn_metadata.trait_name(),
        }
    }
}
pub const trait TraitObjectDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: TraitObjectConstDynMetadata>(&self, const_dyn_metadata: &T) -> TraitObjectMetadata {
        TraitObjectMetadata {
            id_path: const_dyn_metadata.id_path(),
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            trait_object_name: const_dyn_metadata.trait_object_name(),
        }
    }
}
// Types
pub const trait TypeDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: TypeConstDynMetadata>(&self, const_dyn_metadata: &T) -> TypeMetadata {
        TypeMetadata {
            id_path: const_dyn_metadata.id_path(),
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            
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
            id_path: const_dyn_metadata.id_path(),
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            
            constructor_functions: const_dyn_metadata.constructor_functions().clone(),
            method_functions: const_dyn_metadata.method_functions().clone(),
        }
    }
}

// Functions
pub trait ModuleAssociatedFunctionDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: ModuleAssociatedFunctionConstDynMetadata>(&self, const_dyn_metadata: &T) -> ModuleAssociatedFunctionMetadata {
        ModuleAssociatedFunctionMetadata {
            id_path: const_dyn_metadata.id_path(),
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            registrator: const_dyn_metadata.registrator(),
        }
    }
}
pub trait ItemAssociatedFunctionDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: ItemAssociatedFunctionConstDynMetadata>(&self, const_dyn_metadata: &T) -> ItemAssociatedFunctionMetadata {
        ItemAssociatedFunctionMetadata {
            id_path: const_dyn_metadata.id_path(),
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            registrator: const_dyn_metadata.registrator(),
        }
    }
}
pub trait ConstructorFunctionDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: ConstructorFunctionConstDynMetadata>(&self, const_dyn_metadata: &T) -> ConstructorFunctionMetadata {
        ConstructorFunctionMetadata {
            id_path: const_dyn_metadata.id_path(),
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            registrator: const_dyn_metadata.registrator(),
        }
    }
}
pub trait MethodFunctionDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: MethodFunctionConstDynMetadata>(&self, const_dyn_metadata: &T) -> MethodFunctionMetadata {
        MethodFunctionMetadata {
            id_path: const_dyn_metadata.id_path(),
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            registrator: const_dyn_metadata.registrator(),
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






