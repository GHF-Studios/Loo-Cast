//! Generic arbitration for UI/tools that temporarily own pointer or gameplay input.
//!
//! This deliberately has no knowledge of debugging, inventories, pause menus, editors,
//! consoles, or the player. Those systems claim focus by stable owner ID; input adapters
//! only ask whether a class of input is currently claimed.

use std::collections::HashSet;

use bevy::prelude::*;

pub type InputFocusOwner = &'static str;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputFocusSet {
    /// Consumers resolve the claims written earlier in the frame.
    Resolve,
}

#[derive(Resource, Debug, Default)]
pub struct InputFocus {
    pointer_claims: HashSet<InputFocusOwner>,
    gameplay_claims: HashSet<InputFocusOwner>,
    /// Monotonic return-to-gameplay intent observed by concrete input adapters.
    gameplay_resume_epoch: u64,
}

impl InputFocus {
    pub fn set_pointer_claim(&mut self, owner: InputFocusOwner, claimed: bool) {
        set_claim(&mut self.pointer_claims, owner, claimed);
    }

    pub fn set_gameplay_claim(&mut self, owner: InputFocusOwner, claimed: bool) {
        set_claim(&mut self.gameplay_claims, owner, claimed);
    }

    pub fn set_modal_claim(&mut self, owner: InputFocusOwner, claimed: bool) {
        self.set_pointer_claim(owner, claimed);
        self.set_gameplay_claim(owner, claimed);
    }

    pub fn pointer_claimed(&self) -> bool {
        !self.pointer_claims.is_empty()
    }

    pub fn gameplay_claimed(&self) -> bool {
        !self.gameplay_claims.is_empty()
    }

    pub fn request_gameplay_resume(&mut self) {
        self.gameplay_resume_epoch = self.gameplay_resume_epoch.wrapping_add(1);
    }

    pub fn gameplay_resume_epoch(&self) -> u64 {
        self.gameplay_resume_epoch
    }

}

fn set_claim(claims: &mut HashSet<InputFocusOwner>, owner: InputFocusOwner, claimed: bool) {
    if claimed {
        claims.insert(owner);
    } else {
        claims.remove(owner);
    }
}
