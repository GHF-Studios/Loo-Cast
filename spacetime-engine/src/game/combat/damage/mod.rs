//! Hit-to-damage conversion.

use super::*;

pub(super) fn hits_to_damage(
    mut hits: MessageReader<Hit>,
    manifestations: Query<&UsfManifestationOf>,
    mut damage: MessageWriter<Damage>,
) {
    for hit in hits.read() {
        let target = manifestations
            .get(hit.target)
            .map(|manifestation| manifestation.0)
            .unwrap_or(hit.target);

        damage.write(Damage {
            target,
            instigator: Some(hit.instigator),
            amount: hit.damage,
        });
    }
}
