use rhai::{FuncRegistration, Shared};
use std::sync::RwLock;

use crate::player::bundles::PlayerBundle as NativePlayerBundle;
use crate::rhai_binding::meta::abstract_::trait_identity::ToTraitObject;
use crate::rhai_binding::value_semantics::access_cell::{AccessCell, Persistent};
use crate::rhai_binding::value_semantics::modes::{GetTypeValueSemantics, TypeValueSemantics};
use crate::rhai_binding::value_semantics::scoped_access::{ScopedAccess, ScopedAccessHandle};
use crate::rhai_binding::value_semantics::trait_object::StaticTraitObject;
use crate::script::ecs::bundle::internals::trait_objects::{BundleTrait, BundleTraitObject};

type OwnedPlayerBundle = AccessCell<Persistent, NativePlayerBundle>;
type ScopedPlayerBundle = ScopedAccessHandle<NativePlayerBundle>;

core_mod_macros::reflect_sub_module!(
    id = player::bundles,
    sub_modules = [],
    traits = [],
    types = [PlayerBundle],
    module_associated_functions = [],
);

core_mod_macros::reflect_extern_type!(
    id = player::bundles::PlayerBundle,
    rust_type = OwnedPlayerBundle,
    value_semantics = owned,
    method_functions = [player::bundles::PlayerBundle::test_print],
    item_associated_functions = [
        player::bundles::PlayerBundle::new_default,
        player::bundles::PlayerBundle::as_trait_obj,
    ],
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        match <NativePlayerBundle as GetTypeValueSemantics>::VALUE_SEMANTICS {
            TypeValueSemantics::Owned => {
                parent_module.set_custom_type::<OwnedPlayerBundle>(&name);
            }
            TypeValueSemantics::ScopedMut => {
                parent_module.set_custom_type::<ScopedPlayerBundle>(&name);
            }
            TypeValueSemantics::Clone
            | TypeValueSemantics::Ref
            | TypeValueSemantics::Mut
            | TypeValueSemantics::ScopedOwned
            | TypeValueSemantics::ScopedRef => {
                panic!(
                    "PlayerBundle bindings currently support only 'owned' and 'scoped_mut'"
                )
            }
        }
    },
);

core_mod_macros::reflect_extern_item_associated_function!(
    id = player::bundles::PlayerBundle::as_trait_obj,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        match <NativePlayerBundle as GetTypeValueSemantics>::VALUE_SEMANTICS {
            TypeValueSemantics::Owned => {
                FuncRegistration::new(name).set_into_module(
                    parent_module,
                    |bundle: OwnedPlayerBundle, trait_id: &str| match trait_id {
                        "ecs::bundle::Bundle" => {
                            let b: StaticTraitObject<BundleTrait> = bundle.cast_to();
                            BundleTraitObject(b)
                        }
                        unknown_trait_id => panic!("Unknown trait id: '{unknown_trait_id}'"),
                    },
                );
            }
            TypeValueSemantics::ScopedMut => {
                FuncRegistration::new(name).set_into_module(
                    parent_module,
                    |bundle: ScopedPlayerBundle, trait_id: &str| match trait_id {
                        "ecs::bundle::Bundle" => {
                            let b: StaticTraitObject<BundleTrait> = bundle.cast_to();
                            BundleTraitObject(b)
                        }
                        unknown_trait_id => panic!("Unknown trait id: '{unknown_trait_id}'"),
                    },
                );
            }
            TypeValueSemantics::Clone
            | TypeValueSemantics::Ref
            | TypeValueSemantics::Mut
            | TypeValueSemantics::ScopedOwned
            | TypeValueSemantics::ScopedRef => {
                panic!(
                    "PlayerBundle bindings currently support only 'owned' and 'scoped_mut'"
                )
            }
        }
    },
);

core_mod_macros::reflect_extern_item_associated_function!(
    id = player::bundles::PlayerBundle::new_default,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        match <NativePlayerBundle as GetTypeValueSemantics>::VALUE_SEMANTICS {
            TypeValueSemantics::Owned => {
                FuncRegistration::new(name).set_into_module(parent_module, || -> OwnedPlayerBundle {
                    AccessCell::new(NativePlayerBundle::default())
                });
            }
            TypeValueSemantics::ScopedMut => {
                FuncRegistration::new(name).set_into_module(parent_module, || -> ScopedPlayerBundle {
                    ScopedAccessHandle(Shared::new(RwLock::new(ScopedAccess::new(Box::new(
                        NativePlayerBundle::default(),
                    )))))
                });
            }
            TypeValueSemantics::Clone
            | TypeValueSemantics::Ref
            | TypeValueSemantics::Mut
            | TypeValueSemantics::ScopedOwned
            | TypeValueSemantics::ScopedRef => {
                panic!(
                    "PlayerBundle bindings currently support only 'owned' and 'scoped_mut'"
                )
            }
        }
    },
);

core_mod_macros::reflect_extern_method_function!(
    id = player::bundles::PlayerBundle::test_print,
    registrator = |name: rhai::ImmutableString, engine: &mut rhai::Engine| {
        match <NativePlayerBundle as GetTypeValueSemantics>::VALUE_SEMANTICS {
            TypeValueSemantics::Owned => {
                engine.register_fn(name, |b: OwnedPlayerBundle| {
                    let guard = b.start_read();
                    guard.test_print();
                    b.end_read(guard);
                });
            }
            TypeValueSemantics::ScopedMut => {
                engine.register_fn(name, |b: ScopedPlayerBundle| {
                    b.0.read().unwrap().read(|bundle| bundle.test_print()).unwrap();
                });
            }
            TypeValueSemantics::Clone
            | TypeValueSemantics::Ref
            | TypeValueSemantics::Mut
            | TypeValueSemantics::ScopedOwned
            | TypeValueSemantics::ScopedRef => {
                panic!(
                    "PlayerBundle bindings currently support only 'owned' and 'scoped_mut'"
                )
            }
        }
    },
);
