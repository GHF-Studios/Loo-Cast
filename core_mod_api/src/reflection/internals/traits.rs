#![allow(clippy::missing_safety_doc)]

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
pub trait GetTraitId: Clone + PartialEq + Eq + Hash + Sized + 'static {
    const TRAIT_NAME: &'static str;
    const TRAIT_OBJECT_NAME: &'static str;
    const TRAIT_ID: &'static str;
}
pub trait ToTraitObject<T: GetTraitId>: Sized {
    fn cast_to(self) -> StaticTraitObject<T>;
    fn cast_from(obj: StaticTraitObject<T>) -> Self;
}


use crate::reflection::{
    ids::TypeId,
    type_info::TypeInfo,
};
use crate::reflection::internals::statics::{TYPE_REGISTRY, CTOR_REGISTRY, METHOD_REGISTRY, STATIC_FUNCTION_REGISTRY};
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
    inventory::submit!(TopLevelModuleMetadata {
        id_thunk: || "shop".into(),
        inner_thunk: || Box::new(__Shop__TopLevelModule__) as Box<dyn TopLevelModule>,
    });

    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct __Shop__TopLevelModule__;
    impl Reflect for __Shop__TopLevelModule__ {
        fn rust_module_path(&self) -> &'static str { module_path!() }
    }
    impl SubModuleContainer for __Shop__TopLevelModule__ {
        fn sub_modules(&self) -> Vec<Box<dyn SubModule>> {
            vec![
                Box::new(divisions::__Divisions__SubModule__)
            ]
        }
    }
    impl TraitContainer for __Shop__TopLevelModule__ {
        fn traits(&self) -> Vec<Box<dyn Trait>> {
            vec![]
        }
    }
    impl TypeContainer for __Shop__TopLevelModule__ {
        fn types(&self) -> Vec<(Box<dyn Type>, Box<dyn TypeProxyModule>)> {
            vec![]
        }
    }
    impl ModuleAssociatedFunctionContainer for __Shop__TopLevelModule__ {
        fn module_associated_functions(&self) -> Vec<Box<dyn ModuleAssociatedFunction>> {
            vec![]
        }
    }
    impl TopLevelModule for __Shop__TopLevelModule__ {
        fn id_path(&self) -> TopLevelModulePath { "shop".into() }
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
        inventory::submit!(SubModuleMetadata {
            id_thunk: || "shop::divisions".into(),
            inner_thunk: || Box::new(__Divisions__SubModule__) as Box<dyn SubModule>,
        });

        #[derive(Clone, PartialEq, Eq, Hash)]
        pub struct __Divisions__SubModule__;
        impl Reflect for __Divisions__SubModule__ {
            fn rust_module_path(&self) -> &'static str { module_path!() }
        }
        impl SubModuleContainer for __Divisions__SubModule__ {
            fn sub_modules(&self) -> Vec<Box<dyn SubModule>> {
                vec![
                    Box::new(sex::__Sex__SubModule__)
                ]
            }
        }
        impl TraitContainer for __Divisions__SubModule__ {
            fn traits(&self) -> Vec<Box<dyn Trait>> {
                vec![]
            }
        }
        impl TypeContainer for __Divisions__SubModule__ {
            fn types(&self) -> Vec<(Box<dyn Type>, Box<dyn TypeProxyModule>)> {
                vec![]
            }
        }
        impl ModuleAssociatedFunctionContainer for __Divisions__SubModule__ {
            fn module_associated_functions(&self) -> Vec<Box<dyn ModuleAssociatedFunction>> {
                vec![]
            }
        }
        impl SubModule for __Divisions__SubModule__ {
            fn id_path(&self) -> SubModulePath { "shop::divisions".into() }
        }
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> //

// The actual end-user code
// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< //
        pub mod sex {
            use crate::{reflection::internals::traits::*, utils::string::MethodFunctionPath};
            use core_mod_macros::{
                reflect_sub_module,
                reflect_trait,
                reflect_type,
                reflect_module_associated_function,
                reflect_type_associated_function,
                reflect_constructor_function,
                reflect_method_function
            };

            reflect_sub_module!(
                id = shop::divisions::sex,
                sub_modules = [],
                traits = [SexShopTest],
                types = [SexShopProduct],
                module_associated_functions = [test_function],
            );

            #[reflect_trait(id = shop::divisions::sex::SexShopTest)]
            pub trait SexShopTest {
                fn test();
            }
            
            #[reflect_type(id = shop::divisions::sex::SexShopProduct)]
            #[derive(Clone)]
            pub struct SexShopProduct {
                name: &'static str,
                price_usd: f32,
            }
            #[reflect_inherent_impl(id = shop::divisions::sex::SexShopProduct)]
            impl SexShopProduct {
                #[reflect_constructor_function(inherent)]
                pub fn new(name: &'static str, price_usd: f32) -> Self { Self { name, price_usd } }

                #[reflect_method_function(inherent)]
                pub fn name(&self) -> &'static str { self.name }

                #[reflect_method_function(inherent)]
                pub fn price_usd(&self) -> f32 { self.price_usd }

                #[reflect_type_associated_function(inherent)]
                pub fn verify_price(price_usd: f32) -> Result<(), ()> {
                    if price_usd >= 0.0 { Ok(()) } else { Err(()) }
                }
            }
            #[reflect_trait_impl(trait_id = shop::divisions::sex::SexShopTest, type_id = shop::divisions::sex::SexShopProduct)]
            impl SexShopTest for SexShopProduct {
                #[reflect_type_associated_function(via_trait)]
                fn test() {
                    println!("Small banana sound!")
                }
            }

            #[reflect_module_associated_function(id = shop::divisions::sex::test_function)]
            pub fn test_function() {
                println!("Big paling sound!")
            }
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> //

// Expanded MetaProgramming Magic (It's just a bunch of metadata)
// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< //
            inventory::submit!(SubModuleMetadata {
                id_thunk: || "shop::divisions::sex".into(),
                inner_thunk: || Box::new(__Sex__SubModule__) as Box<dyn SubModule>,
            });
            inventory::submit!(TraitMetadata {
                id_thunk: || "shop::divisions::sex::SexShopTest".into(),
                inner_thunk: || Box::new(__SexShopTest__Trait__) as Box<dyn Trait>,
            });
            inventory::submit!(TraitObjectMetadata {
                id_thunk: || "shop::divisions::sex::SexShopTest".into(),
                inner_thunk: || Box::new(__SexShopTest__TraitObject__) as Box<dyn TraitObject>,
            });
            inventory::submit!(TypeMetadata {
                id_thunk: || "shop::divisions::sex::SexShopProduct".into(),
                inner_thunk: || Box::new(__SexShopProduct__Type__) as Box<dyn Type>,
            });
            inventory::submit!(ModuleAssociatedFunctionMetadata {
                id_thunk: || "shop::divisions::sex::test_function".into(),
                inner_thunk: || Box::new(__TestFunction__ModuleAssociatedFunction__) as Box<dyn ModuleAssociatedFunction>,
            });
            inventory::submit!(TypeAssociatedFunctionMetadata {
                id_thunk: || "shop::divisions::sex::verify_price".into(),
                inner_thunk: || Box::new(__VerifyPrice__TypeAssociatedFunction__) as Box<dyn TypeAssociatedFunction>,
            });
            inventory::submit!(ConstructorFunctionMetadata {
                id_thunk: || "shop::divisions::sex::new".into(),
                inner_thunk: || Box::new(__New__ConstructorFunction__) as Box<dyn ConstructorFunction>,
            });
            inventory::submit!(MethodFunctionMetadata {
                id_thunk: || "shop::divisions::sex::name".into(),
                inner_thunk: || Box::new(__Name__MethodFunction__) as Box<dyn MethodFunction>,
            });
            inventory::submit!(MethodFunctionMetadata {
                id_thunk: || "shop::divisions::sex::price_usd".into(),
                inner_thunk: || Box::new(__PriceUsd__MethodFunction__) as Box<dyn MethodFunction>,
            });

            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __Sex__SubModule__;
            impl Reflect for __Sex__SubModule__ {
                fn rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl SubModuleContainer for __Sex__SubModule__ {
                fn sub_modules(&self) -> Vec<Box<dyn SubModule>> {
                    vec![]
                }
            }
            impl TraitContainer for __Sex__SubModule__ {
                fn traits(&self) -> Vec<Box<dyn Trait>> {
                    vec![
                        Box::new(__SexShopTest__Trait__)
                    ]
                }
            }
            impl TypeContainer for __Sex__SubModule__ {
                fn types(&self) -> Vec<(Box<dyn Type>, Box<dyn TypeProxyModule>)> {
                    vec![
                        (Box::new(__SexShopProduct__Type__), Box::new(__SexShopProduct__TypeProxyModule__))
                    ]
                }
            }
            impl ModuleAssociatedFunctionContainer for __Sex__SubModule__ {
                fn module_associated_functions(&self) -> Vec<Box<dyn ModuleAssociatedFunction>> {
                    vec![
                        Box::new(__TestFunction__ModuleAssociatedFunction__)
                    ]
                }
            }
            impl SubModule for __Sex__SubModule__ {
                fn id_path(&self) -> SubModulePath { "shop::divisions::sex".into() }
                fn register_sub_module(&self, _engine: &mut rhai::Engine, _parent_module: &mut rhai::Module) {}
            }

            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __SexShopTest__Trait__;
            impl Reflect for __SexShopTest__Trait__ {
                fn rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl GetTraitId for __SexShopTest__Trait__ {
                const TRAIT_NAME: &'static str = "SexShopTest";
                const TRAIT_OBJECT_NAME: &'static str = "SexShopTestTraitObject";
                const TRAIT_ID: &'static str = "shop::divisions::sex::SexShopTest";
            }
            impl Trait for __SexShopTest__Trait__ {
                fn id_path(&self) -> TraitPath { "shop::divisions::sex::SexShopTest".into() }

                fn register_trait(&self, parent_module: &mut rhai::Module) {
                    parent_module.set_custom_type::<__SexShopTest__Trait__>(Self::TRAIT_NAME);
                    parent_module.set_custom_type::<SexShopTestTraitObject>(Self::TRAIT_OBJECT_NAME);
                }
            }

            // TODO: WIP! Implement properly; this is highly experimental!
            #[repr(transparent)]
            pub struct SexShopTestTraitObject(pub StaticTraitObject<__SexShopTest__Trait__>);

            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __SexShopTest__TraitObject__;
            impl Reflect for __SexShopTest__TraitObject__ {
                fn rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl TraitObject for __SexShopTest__TraitObject__ {
                fn id_path(&self) -> TraitPath { "shop::divisions::sex::SexShopTestTraitObject".into() }
            }

            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __SexShopProduct__Type__;
            impl Reflect for __SexShopProduct__Type__ {
                fn rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl Type for __SexShopProduct__Type__ {
                fn id(&self) -> TypePath { "shop::divisions::sex::SexShopProduct".into() }
            }
            impl MethodFunctionContainer for __SexShopProduct__Type__ {
                fn method_functions(&self) -> Vec<Box<dyn MethodFunction>> {
                    vec![
                        Box::new(__Name__MethodFunction__) as Box<dyn MethodFunction>,
                        Box::new(__PriceUsd__MethodFunction__) as Box<dyn MethodFunction>
                    ]
                }
            }

            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __SexShopProduct__TypeProxyModule__;
            impl Reflect for __SexShopProduct__TypeProxyModule__ {
                fn rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl ConstructorFunctionContainer for __SexShopProduct__TypeProxyModule__ {
                fn constructor_functions(&self) -> Vec<Box<dyn ConstructorFunction>> {
                    vec![
                        Box::new(__New__ConstructorFunction__)
                    ]
                }
            }
            impl TypeAssociatedFunctionContainer for __SexShopProduct__TypeProxyModule__ {
                fn type_associated_functions(&self) -> Vec<Box<dyn TypeAssociatedFunction>> {
                    vec![
                        Box::new(__VerifyPrice__TypeAssociatedFunction__)
                    ]
                }
            }
            impl TypeProxyModule for __SexShopProduct__TypeProxyModule__ {
                fn id_path(&self) -> TypeProxyModulePath { "shop::divisions::sex::SexShopProduct".into() }
            }

            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __TestFunction__ModuleAssociatedFunction__;
            impl Reflect for __TestFunction__ModuleAssociatedFunction__ {
                fn rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl ModuleAssociatedFunction for __TestFunction__ModuleAssociatedFunction__ {
                fn id_path(&self) -> ModuleAssociatedFunctionPath { "shop::divisions::sex::test_function".into() }
                fn get_registrator(&self) -> Box<dyn FnOnce(&mut rhai::Module)> {
                    let name = self.id_path().function_name().clone();
                    let func = move |parent_module: &mut rhai::Module| {
                        rhai::FuncRegistration::new(name)
                            .set_into_module(parent_module, test_function);
                    };
                    Box::new(func)
                }
            }

            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __VerifyPrice__TypeAssociatedFunction__;
            impl Reflect for __VerifyPrice__TypeAssociatedFunction__ {
                fn rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl TypeAssociatedFunction for __VerifyPrice__TypeAssociatedFunction__ {
                fn id_path(&self) -> TypeAssociatedFunctionPath { "shop::divisions::sex::SexShopProduct::verify_price".into() }
                fn get_registrator(&self) -> Box<dyn FnOnce(&mut rhai::Module)> {
                    let name = self.id_path().function_name().clone();
                    let func = move |parent_module: &mut rhai::Module| {
                        rhai::FuncRegistration::new(name)
                            .set_into_module(parent_module, SexShopProduct::verify_price);
                    };
                    Box::new(func)
                }
            }

            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __New__ConstructorFunction__;
            impl Reflect for __New__ConstructorFunction__ {
                fn rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl ConstructorFunction for __New__ConstructorFunction__ {
                fn id_path(&self) -> ConstructorFunctionPath { "shop::divisions::sex::SexShopProduct::new".into() }
                fn get_registrator(&self) -> Box<dyn FnOnce(&mut rhai::Module)> {
                    let name = self.id_path().function_name().clone();
                    let func = move |parent_module: &mut rhai::Module| {
                        rhai::FuncRegistration::new(name)
                            .set_into_module(parent_module, SexShopProduct::new);
                    };
                    Box::new(func)
                }
            }

            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __Name__MethodFunction__;
            impl Reflect for __Name__MethodFunction__ {
                fn rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl MethodFunction for __Name__MethodFunction__ {
                fn id_path(&self) -> MethodFunctionPath { "shop::divisions::sex::SexShopProduct::name".into() }
                fn get_registrator(&self) -> Box<dyn FnOnce(&mut rhai::Engine)> {
                    let name = self.id_path().function_name().clone();
                    let func = move |engine: &mut rhai::Engine| {
                        engine.register_fn(name, SexShopProduct::name);
                    };
                    Box::new(func)
                }
            }
            
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct __PriceUsd__MethodFunction__;
            impl Reflect for __PriceUsd__MethodFunction__ {
                fn rust_module_path(&self) -> &'static str { module_path!() }
            }
            impl MethodFunction for __PriceUsd__MethodFunction__ {
                fn id_path(&self) -> MethodFunctionPath { "shop::divisions::sex::SexShopProduct::price_usd".into() }
                fn get_registrator(&self) -> Box<dyn FnOnce(&mut rhai::Engine)> {
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



// Module Metadata
inventory::collect!(TopLevelModuleMetadata);
#[derive(Clone, Copy)]
pub struct TopLevelModuleMetadata {
    pub id_thunk: fn() -> TopLevelModulePath,
    pub inner_thunk: fn() -> Box<dyn TopLevelModule>
}
inventory::collect!(SubModuleMetadata);
#[derive(Clone, Copy)]
pub struct SubModuleMetadata {
    pub id_thunk: fn() -> SubModulePath,
    pub inner_thunk: fn() -> Box<dyn SubModule>
}
inventory::collect!(TypeProxyModuleMetadata);
#[derive(Clone, Copy)]
pub struct TypeProxyModuleMetadata {
    pub id_thunk: fn() -> TypeProxyModulePath,
    pub inner_thunk: fn() -> Box<dyn TypeProxyModule>
}

// Trait Metadata
inventory::collect!(TraitMetadata);
#[derive(Clone, Copy)]
pub struct TraitMetadata {
    pub id_thunk: fn() -> TraitPath,
    pub inner_thunk: fn() -> Box<dyn Trait>
}
inventory::collect!(TraitObjectMetadata);
#[derive(Clone, Copy)]
pub struct TraitObjectMetadata {
    pub id_thunk: fn() -> TraitPath,
    pub inner_thunk: fn() -> Box<dyn TraitObject>
}

// Type Metadata
inventory::collect!(TypeMetadata);
#[derive(Clone, Copy)]
pub struct TypeMetadata {
    pub id_thunk: fn() -> TypePath,
    pub inner_thunk: fn() -> Box<dyn Type>
}

// Function Metadata
inventory::collect!(ModuleAssociatedFunctionMetadata);
#[derive(Clone, Copy)]
pub struct ModuleAssociatedFunctionMetadata {
    pub id_thunk: fn() -> ModuleAssociatedFunctionPath,
    pub inner_thunk: fn() -> Box<dyn ModuleAssociatedFunction>
}
inventory::collect!(TypeAssociatedFunctionMetadata);
#[derive(Clone, Copy)]
pub struct TypeAssociatedFunctionMetadata {
    pub id_thunk: fn() -> TypeAssociatedFunctionPath,
    pub inner_thunk: fn() -> Box<dyn TypeAssociatedFunction>
}
inventory::collect!(ConstructorFunctionMetadata);
#[derive(Clone, Copy)]
pub struct ConstructorFunctionMetadata {
    pub id_thunk: fn() -> ConstructorFunctionPath,
    pub inner_thunk: fn() -> Box<dyn ConstructorFunction>
}
inventory::collect!(MethodFunctionMetadata);
#[derive(Clone, Copy)]
pub struct MethodFunctionMetadata {
    pub id_thunk: fn() -> MethodFunctionPath,
    pub inner_thunk: fn() -> Box<dyn MethodFunction>
}


pub trait Reflect: 'static + Send + Sync {
    fn rust_module_path(&self) -> &'static str;
}

// Module
pub trait TopLevelModule: Reflect + SubModuleContainer + TraitContainer + TypeContainer + ModuleAssociatedFunctionContainer {
    /// Format: "some_sort_of::path::to::my_module"
    fn id_path(&self) -> TopLevelModulePath;

    fn register_top_level_module(&self, engine: &mut rhai::Engine) {
        let mut top_level_module = rhai::Module::new();
        top_level_module.set_id(self.id_path().module_name());

        for sub_module in self.sub_modules().into_iter() {
            sub_module.register_sub_module(engine, &mut top_level_module);
        }

        for trait_ in self.traits().into_iter() {
            trait_.register_trait(&mut top_level_module);
        }

        for (type_, type_module) in self.types().into_iter() {
            type_.register_type(engine, &mut top_level_module);
            type_module.register_type_proxy_module(&mut top_level_module);
        }

        for module_associated_function in self.module_associated_functions().into_iter() {
            module_associated_function.register_module_associated_function(&mut top_level_module);
        }

        engine.register_static_module(self.id_path().module_name(), Arc::new(top_level_module));
    }
}
pub trait SubModule: Reflect + SubModuleContainer + TraitContainer + TypeContainer + ModuleAssociatedFunctionContainer {
    /// Format: "some_sort_of::path::to::my_module"
    fn id_path(&self) -> SubModulePath;

    fn register_sub_module(&self, engine: &mut rhai::Engine, parent_module: &mut rhai::Module) {
        let mut origin_sub_module = rhai::Module::new();
        origin_sub_module.set_id(self.id_path().module_name());

        for sub_module in self.sub_modules().into_iter() {
            sub_module.register_sub_module(engine, &mut origin_sub_module);
        }

        for trait_ in self.traits().into_iter() {
            trait_.register_trait(&mut origin_sub_module);
        }

        for (type_, type_module) in self.types().into_iter() {
            type_.register_type(engine, &mut origin_sub_module);
            type_module.register_type_proxy_module(&mut origin_sub_module);
        }

        for module_associated_function in self.module_associated_functions().into_iter() {
            module_associated_function.register_module_associated_function(&mut origin_sub_module);
        }

        parent_module.set_sub_module(self.id_path().module_name(), origin_sub_module);
    }
}
pub trait TypeProxyModule: Reflect + TypeAssociatedFunctionContainer + ConstructorFunctionContainer {
    /// Format: "some_sort_of::path::to::MyType"
    fn id_path(&self) -> TypeProxyModulePath;

    fn register_type_proxy_module(&self, parent_module: &mut rhai::Module) {
        let mut type_module = rhai::Module::new();
        type_module.set_id(self.id_path().type_name());

        for type_associated_function in self.type_associated_functions().into_iter() {
            type_associated_function.register_type_associated_function(&mut type_module);
        }

        for constructor_function in self.constructor_functions().into_iter() {
            constructor_function.register_constructor_function(&mut type_module);
        }

        parent_module.set_sub_module(self.id_path().type_name(), type_module);
    }
}

// Trait
/// # How to make a rust-trait rhai-compatible:
/// 
/// Say:
/// ```
/// pub trait Foo {}
/// ```
/// 
/// Then we would proceed to make Foo rhai-compatible like this:
/// ```
/// use rhai::Dynamic;
/// 
/// use crate::{
///     reflection::{
///         ids::{StaticTraitId, TypeId},
///         internals::traits::{ToTraitObject, GetTraitId},
///         traits::StaticTraitObject
///     },
///     script::access::ScopedAccessHandle,
/// };
/// 
/// #[derive(Clone, PartialEq, Eq, Hash)]
/// pub struct __Foo__Trait__;
/// impl Trait for __Foo__Trait__ {
///     fn name(&self) -> &'static str { "Foo" }
/// 
///     fn register_trait(&self, parent_module: &mut rhai::Module) {
///         parent_module.set_custom_type::<__Foo__Trait__>(__Foo__Trait__::name());
///         parent_module.set_custom_type::<__Foo__TraitObject__>(__Foo__TraitObject__::name());
///     }
/// }
/// 
/// #[repr(transparent)]
/// pub struct __Foo__TraitObject__(pub StaticTraitObject<FooTrait>);
/// ```
pub trait Trait: Reflect {
    /// Format: "some_sort_of::path::to::MyTrait"
    fn id_path(&self) -> TraitPath;
    fn register_trait(&self, parent_module: &mut rhai::Module);
}
/// # How to make a rust-trait-implementor rhai-compatible:
/// 
/// Say:
/// ```
/// pub trait Foo {}
/// 
/// pub struct Bar;
/// impl Foo for Bar {}
/// 
/// pub struct Baz;
/// impl Foo for Baz {}
/// ```
/// 
/// And assuming Foo has already been made rhai-compatible (see the `Trait` documentation for more info)
/// 
/// Then we would simply proceed to implement ToTraitObject<Foo> for Bar and Baz like this:
/// ```
/// use rhai::Dynamic;
/// 
/// use crate::{
///     reflection::{
///         ids::{StaticTraitId, TypeId},
///         internals::traits::ToTraitObject,
///         traits::StaticTraitObject
///     },
///     script::access::ScopedAccessHandle,
/// };
/// 
/// 
/// impl ToTraitObject<__Foo__Trait__> for ScopedAccessHandle<Bar> {
///     fn cast_to(self) -> StaticTraitObject<__Foo__Trait__> {
///         StaticTraitObject {
///             value: Dynamic::from(self.0),
///             trait_id: StaticTraitId::new(),
///             instance_type_id: TypeId::of::<Bar>(),
///         }
///     }
/// 
///     fn cast_from(obj: StaticTraitObject<__Foo__Trait__>) -> Self {
///         ScopedAccessHandle(obj.value.cast())
///     }
/// }
/// ```
pub trait TraitObject: Reflect {
    /// Format: "some_sort_of::path::to::MyTrait"
    fn id_path(&self) -> TraitPath;
}

// Type
/// I think this is outdated, and the entire Type shit is not yet adapted to the new reflection paradigm, aka there are no metadata structs yet
pub trait Type: Reflect + MethodFunctionContainer {
    /// Format: "some_sort_of::path::to::MyType"
    fn id(&self) -> TypePath;

    fn register_type(&self, engine: &mut rhai::Engine, parent_module: &mut rhai::Module) {
        parent_module.set_custom_type::<ScopedAccessHandle<Self>>(self.id().type_name());

        for method_function in self.method_functions().into_iter() {
            method_function.register_method_function(engine);
        }
    }
}
pub trait TypeOwn: Type {}
pub trait TypeClone: Type {}
pub trait TypePersistentRef: Type {}
pub trait TypePersistentMut: Type {}
/// Like a PersistentRef, but backs a rust-native immutable borrow *with* lifetimes, aka it implements runtime-checks against use-after-free's and aliasing issues; 
pub trait TypeScopedRef: Type {}
pub trait TypeScopedMut: Type {}

// Function
// (This could be much nicer if rhai publicly re-exported `SendSync`, cause without that we cannot replicate the trait bounds of a rhai function, so we need to some thunk-y stuff)
pub trait ModuleAssociatedFunction: Reflect {
    /// Format: "some_sort_of::path::to::my_module_associated_function"
    fn id_path(&self) -> ModuleAssociatedFunctionPath;
    fn get_registrator(&self) -> Box<dyn FnOnce(&mut rhai::Module)>;

    fn register_module_associated_function(&self, parent_module: &mut rhai::Module) {
        (self.get_registrator())(parent_module);
    }
}
pub trait TypeAssociatedFunction: Reflect {
    /// Format: "some_sort_of::path::to::SomeType::my_type_associated_function"
    fn id_path(&self) -> TypeAssociatedFunctionPath;
    fn get_registrator(&self) -> Box<dyn FnOnce(&mut rhai::Module)>;

    fn register_type_associated_function(&self, parent_module: &mut rhai::Module) {
        (self.get_registrator())(parent_module);
    }
}
pub trait ConstructorFunction: Reflect {
    /// Format: "some_sort_of::path::to::SomeType::my_constructor_function"
    fn id_path(&self) -> ConstructorFunctionPath;
    fn get_registrator(&self) -> Box<dyn FnOnce(&mut rhai::Module)>;

    fn register_constructor_function(&self, parent_module: &mut rhai::Module) {
        (self.get_registrator())(parent_module);
    }
}
pub trait MethodFunction: Reflect {
    /// Format: "some_sort_of::path::to::SomeType::my_method_function"
    fn id_path(&self) -> MethodFunctionPath;
    fn get_registrator(&self) -> Box<dyn FnOnce(&mut rhai::Engine)>;

    fn register_method_function(&self, engine: &mut rhai::Engine) {
        (self.get_registrator())(engine);
    }
}



// Container
pub trait SubModuleContainer: Reflect {
    fn sub_modules(&self) -> Vec<Box<dyn SubModule>>;
}
pub trait TraitContainer: Reflect {
    fn traits(&self) -> Vec<Box<dyn Trait>>;
}
pub trait TypeContainer: Reflect {
    fn types(&self) -> Vec<(Box<dyn Type>, Box<dyn TypeProxyModule>)>;
}
pub trait ModuleAssociatedFunctionContainer: Reflect {
    fn module_associated_functions(&self) -> Vec<Box<dyn ModuleAssociatedFunction>>;
}
pub trait TypeAssociatedFunctionContainer: Reflect {
    fn type_associated_functions(&self) -> Vec<Box<dyn TypeAssociatedFunction>>;
}
pub trait ConstructorFunctionContainer: Reflect {
    fn constructor_functions(&self) -> Vec<Box<dyn ConstructorFunction>>;
}
pub trait MethodFunctionContainer: Reflect {
    fn method_functions(&self) -> Vec<Box<dyn MethodFunction>>;
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

/// Metadata provider for reflection + scripting
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






