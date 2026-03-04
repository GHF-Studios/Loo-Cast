//! Compile-time bundle signature catalog.
//!
//! Reserve this module for bundle constructors and insertion signatures that
//! cannot be materialized dynamically at runtime.

use crate::bevy::ecs::world::EntityWorldMut as BevyEntityWorldMut;
use crate::player::bundles::PlayerBundle;
use crate::rhai_binding::meta::abstract_::trait_identity::ToTraitObject;
use crate::rhai_binding::runtime::ecs::bundle::internals::trait_objects::{BundleTrait, BundleTraitObject};
use crate::rhai_binding::runtime::ecs::bundle::internals::types::BundleSpawnDispatchEntry;
use crate::rhai_binding::value_semantics::access_cell::{AccessCell, Persistent, Scoped};
use crate::rhai_binding::value_semantics::modes::{GetTypeValueSemantics, TypeValueSemantics};

pub const BUNDLE_SIG__PLAYER__SPAWN_SINGLE: &str = "BUNDLE_SIG__PLAYER__SPAWN_SINGLE";

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

inventory::submit! {
    BundleSpawnDispatchEntry {
        signature_id: BUNDLE_SIG__PLAYER__SPAWN_SINGLE,
        instance_type_id: "core_mod_api::player::bundles::PlayerBundle",
        trait_id: "bevy::ecs::bundle::Bundle",
        dispatch: dispatch_bundle_sig_player_spawn_single,
    }
}
