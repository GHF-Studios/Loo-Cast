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
        public float laserLength { get; private set; }
        #endregion

        #region Fields
        #endregion

        #region Constructors
        public LaserEmitterWeaponItem(LaserEmitterWeaponItemData data, Stats stats, ITargeting mainTargeting, GameObject originObject) : base(data, stats, mainTargeting, originObject)
        {
            laserLength = data.LaserLength.Value;
        }
        #endregion

        #region Methods
        public override bool TryFire()
        {
            if (canFire)
            {
                canFire = false;
                fireTimer.Start();

                List<Target> targets = AcquireTargets(1, TargetingMode.Closest);
                if (targets == null || targets.Count == 0)
                {
                    return false;
                }
                Target target = targets[0];

                GameObject bulletObject = GameObject.Instantiate(projectilePrefab, originObject.transform.position, Quaternion.identity);
                bulletObject.transform.position += new Vector3(0, 0, 0.1f);
                bulletObject.GetComponent<LaserProjectile>().Initialize(target, originObject, damage, critChance, critDamage, knockback, projectileSpeed, projectileSize, projectileLifetime, piercing, armorPenetration, laserLength);
                soundHandler.SoundShoot();
                return true;
            }
            else
            {
                return false;
            }
        }
        #endregion
    }
}
