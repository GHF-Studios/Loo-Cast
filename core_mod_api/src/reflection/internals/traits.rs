#![allow(clippy::missing_safety_doc)]

use rhai::{RhaiNativeFunc, Shared, Variant};
use std::hash::Hash;
use std::sync::Arc;

use crate::reflection::traits::StaticTraitObject;


pub trait GetTypeId: Sized + 'static {
    const TYPE_ID: &'static str;
}

pub trait GetTraitId: Clone + PartialEq + Eq + Hash + Sized + 'static {
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


// Example shit
pub mod shop {
    pub mod divisions {
        pub mod sex {
            pub struct __SexShopProduct__;
            impl crate::reflection::internals::traits::Type for __SexShopProduct__ {
                const NAME: &'static str = "SexShopProduct";
            }
            impl crate::reflection::internals::traits::MethodFunctionContainer for __SexShopProduct__ {
                fn method_functions() -> Vec<Box<dyn crate::reflection::internals::traits::MethodFunction>> {
                    vec![
                        Box::new(__Name__) as Box<dyn crate::reflection::internals::traits::MethodFunction>,
                        Box::new(__PriceUsd__) as Box<dyn crate::reflection::internals::traits::MethodFunction>
                    ]
                }
            }

            pub struct __TestFunction__;
            impl crate::reflection::internals::traits::Function for __TestFunction__ {
                fn name(&self) -> &'static str { "test_function" }
            }
            impl crate::reflection::internals::traits::ModuleAssociatedFunction for __TestFunction__ {
                fn get_registrator(&self) -> impl Fn(&mut rhai::Module) {
                    let name = crate::reflection::internals::traits::Function::name(self);
                    move |parent_module: &mut rhai::Module| {
                        rhai::FuncRegistration::new(name)
                            .set_into_module(parent_module, test_function);
                    }
                }
            }

            pub struct __VerifyPrice__;
            impl crate::reflection::internals::traits::Function for __VerifyPrice__ {
                fn name(&self) -> &'static str { "verify_price" }
            }
            impl crate::reflection::internals::traits::TypeAssociatedFunction for __VerifyPrice__ {
                fn get_registrator(&self) -> impl Fn(&mut rhai::Module) {
                    let name = crate::reflection::internals::traits::Function::name(self);
                    move |parent_module: &mut rhai::Module| {
                        rhai::FuncRegistration::new(name)
                            .set_into_module(parent_module, SexShopProduct::verify_price);
                    }
                }
            }

            pub struct __New__;
            impl crate::reflection::internals::traits::Function for __New__ {
                fn name(&self) -> &'static str { "new" }
            }
            impl crate::reflection::internals::traits::ConstructorFunction for __New__ {
                fn get_registrator(&self) -> impl Fn(&mut rhai::Module) {
                    let name = crate::reflection::internals::traits::Function::name(self);
                    move |parent_module: &mut rhai::Module| {
                        rhai::FuncRegistration::new(name)
                            .set_into_module(parent_module, SexShopProduct::new);
                    }
                }
            }

            pub struct __Name__;
            impl crate::reflection::internals::traits::Function for __Name__ {
                fn name(&self) -> &'static str { "name" }
            }
            impl crate::reflection::internals::traits::MethodFunction for __Name__ {
                fn get_registrator(&self) -> impl Fn(&mut rhai::Engine) {
                    let name = crate::reflection::internals::traits::Function::name(self);
                    move |engine: &mut rhai::Engine| {
                        engine.register_fn(name, SexShopProduct::name);
                    }
                }
            }
            pub struct __PriceUsd__;
            impl crate::reflection::internals::traits::Function for __PriceUsd__ {
                fn name(&self) -> &'static str { "price_usd" }
            }
            impl crate::reflection::internals::traits::MethodFunction for __PriceUsd__ {
                fn get_registrator(&self) -> impl Fn(&mut rhai::Engine) {
                    let name = crate::reflection::internals::traits::Function::name(self);
                    move |engine: &mut rhai::Engine| {
                        engine.register_fn(name, SexShopProduct::price_usd);
                    }
                }
            }
            
            #[derive(Clone)]
            pub struct SexShopProduct {
                name: &'static str,
                price_usd: f32,
            }
            impl SexShopProduct {
                pub fn new(name: &'static str, price_usd: f32) -> Self { Self { name, price_usd } }

                pub fn name(&self) -> &'static str { self.name }

                pub fn price_usd(&self) -> f32 { self.price_usd }

                pub fn verify_price(price_usd: f32) -> Result<(), ()> {
                    if price_usd >= 0.0 { Ok(()) } else { Err(()) }
                }
            }

            pub fn test_function() {
                println!("Big paling sound!")
            }
        }
    }
}






// Module
pub trait TopLevelModule: Sized + SubModuleContainer + TraitContainer + TypeContainer + ModuleAssociatedFunctionContainer {
    /// Format: "snake_case"
    const NAME: &'static str;

    fn register_top_level_module(self, engine: &mut rhai::Engine) {
        let mut top_level_module = rhai::Module::new();
        top_level_module.set_id(Self::NAME);

        for sub_module in Self::sub_modules().into_iter() {
            sub_module.register_sub_module(engine, &mut top_level_module);
        }

        for trait_ in Self::traits().into_iter() {
            trait_.register_trait(&mut top_level_module);
        }

        for (type_, type_module) in Self::types().into_iter() {
            type_.register_type(engine, &mut top_level_module);
            type_module.register_type_associated_module(&mut top_level_module);
        }

        for module_associated_function in Self::module_associated_functions().into_iter() {
            module_associated_function.register_module_associated_function(&mut top_level_module);
        }

        engine.register_static_module(Self::NAME, Arc::new(top_level_module));
    }
}
pub trait SubModule: Sized + SubModuleContainer + TraitContainer + TypeContainer + ModuleAssociatedFunctionContainer {
    /// Format: "snake_case"
    const NAME: &'static str;

    fn register_sub_module(self, engine: &mut rhai::Engine, parent_module: &mut rhai::Module) {
        let mut origin_sub_module = rhai::Module::new();
        origin_sub_module.set_id(Self::NAME);

        for sub_module in Self::sub_modules().into_iter() {
            sub_module.register_sub_module(engine, &mut origin_sub_module);
        }

        for trait_ in Self::traits().into_iter() {
            trait_.register_trait(&mut origin_sub_module);
        }

        for (type_, type_module) in Self::types().into_iter() {
            type_.register_type(engine, &mut origin_sub_module);
            type_module.register_type_associated_module(&mut origin_sub_module);
        }

        for module_associated_function in Self::module_associated_functions().into_iter() {
            module_associated_function.register_module_associated_function(&mut origin_sub_module);
        }

        parent_module.set_sub_module(Self::NAME, origin_sub_module);
    }
}
pub trait TypeAssociatedModule: Sized + TypeAssociatedFunctionContainer + ConstructorFunctionContainer {
    /// Format: "PascalCase"
    const NAME: &'static str;

    fn register_type_associated_module(self, parent_module: &mut rhai::Module) {
        let mut type_module = rhai::Module::new();
        type_module.set_id(Self::NAME);

        for type_associated_function in Self::type_associated_functions().into_iter() {
            type_associated_function.register_type_associated_function(&mut type_module);
        }

        for constructor_function in Self::constructor_functions().into_iter() {
            constructor_function.register_constructor_function(&mut type_module);
        }

        parent_module.set_sub_module(Self::NAME, type_module);
    }
}



// Trait
pub trait TraitObject: Sized {
    /// Format: "PascalCase"
    const NAME: &'static str;
}
pub trait Trait: Sized {
    /// Format: "PascalCase"
    const NAME: &'static str;

    type ObjectType: TraitObject;

    fn register_trait(self, parent_module: &mut rhai::Module) {
        parent_module.set_custom_type::<Self>(Self::NAME);
        parent_module.set_custom_type::<Self::ObjectType>(Self::ObjectType::NAME);
    }
}



// Type
pub trait Type: Sized + MethodFunctionContainer {
    /// Format: "PascalCase"
    const NAME: &'static str;

    fn register_type(self, engine: &mut rhai::Engine, parent_module: &mut rhai::Module) {
        parent_module.set_custom_type::<ScopedAccessHandle<Self>>(Self::NAME);

        for method_function in Self::method_functions().into_iter() {
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
pub trait Function {
    /// Format: "snake_case"
    fn name(&self) -> &'static str;
}
pub trait ModuleAssociatedFunction: Function {
    fn get_registrator(&self) -> impl Fn(&mut rhai::Module);

    fn register_module_associated_function(&self, parent_module: &mut rhai::Module) {
        (self.get_registrator())(parent_module);
    }
}
pub trait TypeAssociatedFunction: Function {
    fn get_registrator(&self) -> impl Fn(&mut rhai::Module);

    fn register_type_associated_function(&self, parent_module: &mut rhai::Module) {
        (self.get_registrator())(parent_module);
    }
}
pub trait ConstructorFunction: Function {
    fn get_registrator(&self) -> impl Fn(&mut rhai::Module);

    fn register_constructor_function(&self, parent_module: &mut rhai::Module) {
        (self.get_registrator())(parent_module);
    }
}
pub trait MethodFunction: Function {
    fn get_registrator(&self) -> impl Fn(&mut rhai::Engine);

    fn register_method_function(&self, engine: &mut rhai::Engine) {
        (self.get_registrator())(engine);
    }
}



// Container
pub trait SubModuleContainer {
    fn sub_modules() -> Vec<impl SubModule>;
}
pub trait TraitContainer {
    fn traits() -> Vec<impl Trait>;
}
pub trait TypeContainer {
    fn types() -> Vec<(impl Type, impl TypeAssociatedModule)>;
}
pub trait ModuleAssociatedFunctionContainer {
    fn module_associated_functions() -> Vec<Box<dyn ModuleAssociatedFunction>>;
}
pub trait TypeAssociatedFunctionContainer {
    fn type_associated_functions() -> Vec<Box<dyn TypeAssociatedFunction>>;
}
pub trait ConstructorFunctionContainer {
    fn constructor_functions() -> Vec<Box<dyn ConstructorFunction>>;
}
pub trait MethodFunctionContainer {
    fn method_functions() -> Vec<Box<dyn MethodFunction>>;
}













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






