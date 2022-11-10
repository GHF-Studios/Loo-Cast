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
            // This works perfectly fine when being manually triggered
            // But when it is fired through autoFire, THROUGH A TIMER, THAT RUNS ASYNCHRONOUSLY TO UNITY UPDATES, IT FUCKS UP
            // TODO: Investigate

            Debug.Log("1.1");
            GameObject originObject = ItemContainer.OriginObject;
            Debug.Log("1.2");
            Transform originTransform = originObject.transform;
            // THE CODE, STARTING HERE, IS NEVER REACHED!
            Debug.Log("1.3");
            Vector3 samplePosition = originTransform.position;
            Debug.Log("2");
            float sampleRadius = Range.Value;
            Debug.Log("3");
            string[] enemyTags = TeamUtil.GetEnemyTags(ItemContainer.OriginObject);
            Debug.Log("4");
            LayerMask enemyLayerMask = TeamUtil.GetEnemyLayerMask(ItemContainer.OriginObject);
            Debug.Log("5");
            Target[] targets = TargetingUtil.GetClosestTargets(samplePosition, sampleRadius, enemyTags, enemyLayerMask);
            Debug.Log("6");
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
