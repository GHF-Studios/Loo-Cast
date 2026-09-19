//! Hit-to-damage conversion and authoritative health/death consequences.

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

pub(super) fn apply_damage(
    mut damage: MessageReader<Damage>,
    mut health: Query<&mut Health>,
    mut died: MessageWriter<Died>,
) {
    for damage in damage.read() {
        let Ok(mut health) = health.get_mut(damage.target) else {
            continue;
        };

        if health.damage(damage.amount) {
            died.write(Died {
                entity: damage.target,
                instigator: damage.instigator,
            });
        }
    }
}
