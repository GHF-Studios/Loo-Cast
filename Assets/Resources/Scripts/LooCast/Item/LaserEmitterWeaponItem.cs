using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item
{
    using Data;
    using LooCast.Attribute.Stat;
    using LooCast.Targeting;
    using LooCast.Target;
    using LooCast.Projectile;

    public class LaserEmitterWeaponItem : WeaponItem
    {
        #region Data
        public LaserEmitterWeaponItemData LaserEmitterWeaponItemData { get; private set; }
        #endregion

        #region Properties
        public float LaserLength { get; private set; }
        #endregion

        #region Fields
        #endregion

        #region Constructors
        public LaserEmitterWeaponItem(LaserEmitterWeaponItemData data, Stats stats, bool autoFire) : base(data, stats, autoFire)
        {
            LaserLength = data.LaserLength.Value;
        }
        #endregion

        #region Methods
        public override void Fire()
        {
            List<Target> targets = AcquireTargets(1, TargetingMode.Closest);
            if (targets == null || targets.Count == 0)
            {
                return;
            }
            Target target = targets[0];

            GameObject bulletObject = GameObject.Instantiate(ProjectilePrefab, ItemContainer.OriginObject.transform.position, Quaternion.identity);
            bulletObject.transform.position += new Vector3(0, 0, 0.1f);
            bulletObject.GetComponent<LaserProjectile>().Initialize(target, ItemContainer.OriginObject, Damage, CritChance, CritDamage, Knockback, ProjectileSpeed, ProjectileSize, ProjectileLifetime, Piercing, ArmorPenetration, LaserLength);
            soundHandler.SoundShoot();
        }
        #endregion
    }
}
