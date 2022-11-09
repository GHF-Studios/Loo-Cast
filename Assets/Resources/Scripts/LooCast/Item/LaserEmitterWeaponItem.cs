using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item
{
    using Data;
    using LooCast.Attribute.Stat;
    using LooCast.Target;
    using LooCast.Projectile;
    using LooCast.Util;
    using LooCast.Variable;

    public class LaserEmitterWeaponItem : WeaponItem
    {
        #region Data
        public LaserEmitterWeaponItemData LaserEmitterWeaponItemData { get; private set; }
        #endregion

        #region Properties
        public FloatComputedVariable LaserLength { get; private set; }
        #endregion

        #region Fields
        #endregion

        #region Constructors
        public LaserEmitterWeaponItem(LaserEmitterWeaponItemData data, Stats stats, bool autoFire) : base(data, stats, autoFire)
        {
            LaserEmitterWeaponItemData = data;

            LaserLength = new FloatComputedVariable(data.LaserLength.Value);
        }
        #endregion

        #region Methods
        public override void Fire()
        {
            Target[] targets = TargetingUtil.GetClosestTargets(ItemContainer.OriginObject.transform.position, Range.Value, TeamUtil.GetEnemyTags(ItemContainer.OriginObject), TeamUtil.GetEnemyLayerMask(ItemContainer.OriginObject));
            if (targets == null || targets.Length == 0)
            {
                return;
            }
            Target target = targets[0];

            GameObject bulletObject = GameObject.Instantiate(ProjectilePrefab, ItemContainer.OriginObject.transform.position, Quaternion.identity);
            bulletObject.transform.position += new Vector3(0, 0, 0.1f);
            bulletObject.GetComponent<LaserProjectile>().Initialize(target, ItemContainer.OriginObject, TeamUtil.GetTeam(ItemContainer.OriginObject.tag), Damage.Value, CritChance.Value, CritDamage.Value, Knockback.Value, ProjectileSpeed.Value, ProjectileSize.Value, ProjectileLifetime.Value, Piercing.Value, ArmorPenetration.Value, LaserLength.Value);
            soundHandler.SoundShoot();
        }
        #endregion
    }
}
