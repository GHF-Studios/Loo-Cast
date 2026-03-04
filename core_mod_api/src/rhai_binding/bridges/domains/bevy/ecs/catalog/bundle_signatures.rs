//! Compile-time bundle signature catalog.
//!
//! Reserve this module for bundle constructors and insertion signatures that
//! cannot be materialized dynamically at runtime.

use crate::bevy::ecs::world::EntityWorldMut as BevyEntityWorldMut;
use crate::player::bundles::PlayerBundle;
use crate::rhai_binding::meta::abstract_::trait_identity::ToTraitObject;
use crate::rhai_binding::runtime::ecs::dispatch_policy::submit_bundle_spawn_dispatch_entry;
use crate::rhai_binding::runtime::ecs::bundle::internals::trait_objects::{BundleTrait, BundleTraitObject};
use crate::rhai_binding::value_semantics::access_cell::{AccessCell, Persistent, Scoped};
use crate::rhai_binding::value_semantics::modes::{GetTypeValueSemantics, TypeValueSemantics};

pub const BUNDLE_SIG__PLAYER__SPAWN_SINGLE: &str = "BUNDLE_SIG__PLAYER__SPAWN_SINGLE";
pub const TYPE_PATH__PLAYER_BUNDLE: &str = "core_mod_api::player::bundles::PlayerBundle";
pub const TYPE_PATH__TRAIT_BUNDLE: &str = "bevy::ecs::bundle::Bundle";

fn dispatch_bundle_sig_player_spawn_single(ent: &mut BevyEntityWorldMut, bundle: BundleTraitObject) {
    match <PlayerBundle as GetTypeValueSemantics>::VALUE_SEMANTICS {
        TypeValueSemantics::ScopedMut => {
            let bundle: AccessCell<Scoped, PlayerBundle> = ToTraitObject::<BundleTrait>::cast_from(bundle.0);
            ent.insert(bundle.take());
        }
        TypeValueSemantics::Owned => {
            let bundle: AccessCell<Persistent, PlayerBundle> = ToTraitObject::<BundleTrait>::cast_from(bundle.0);
            ent.insert(bundle.take());
        }
        TypeValueSemantics::Clone
        | TypeValueSemantics::Ref
        | TypeValueSemantics::Mut
        | TypeValueSemantics::ScopedOwned
        | TypeValueSemantics::ScopedRef => {
            panic!("PlayerBundle spawn dispatch currently supports semantics: owned | scoped_mut")
        }
    }
}

submit_bundle_spawn_dispatch_entry!(
    signature_id = BUNDLE_SIG__PLAYER__SPAWN_SINGLE,
    instance_type_id = TYPE_PATH__PLAYER_BUNDLE,
    trait_id = TYPE_PATH__TRAIT_BUNDLE,
    dispatch = dispatch_bundle_sig_player_spawn_single,
);
