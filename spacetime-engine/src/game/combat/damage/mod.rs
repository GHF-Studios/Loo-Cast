//! Hit-to-damage conversion.

use super::*;

pub(super) fn hits_to_damage(
    mut hits: MessageReader<Hit>,
    ownership: UsfOwnershipQuery,
    mut damage: MessageWriter<Damage>,
) {
    for hit in hits.read() {
        let target = ownership.semantic_of(hit.target).unwrap_or(hit.target);

        damage.write(Damage {
            target,
            instigator: Some(hit.instigator),
            amount: hit.damage,
        });
    }
}
