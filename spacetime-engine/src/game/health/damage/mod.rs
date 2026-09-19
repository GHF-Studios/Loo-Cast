//! Authoritative generic damage application and death emission.

use super::*;

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
