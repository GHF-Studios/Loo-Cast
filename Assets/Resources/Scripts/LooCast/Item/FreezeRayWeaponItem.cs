using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item
{
    using Data;
    using Attribute.Stat;
    using Target;
    using AOE;
    using Util;

    public class FreezeRayWeaponItem : WeaponItem
    {
        #region Data
        public FreezeRayWeaponItemData FreezeRayWeaponItemData { get; private set; }
        #endregion

        #region Properties
        #endregion

        #region Fields
        #endregion

        #region Constructors
        public FreezeRayWeaponItem(FreezeRayWeaponItemData data, Stats stats, bool autoFire) : base(data, stats, autoFire)
        {
            FreezeRayWeaponItemData = data;
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

            GameObject freezeOrbObject = GameObject.Instantiate(ProjectilePrefab, ItemContainer.OriginObject.transform.position, Quaternion.identity);
            freezeOrbObject.transform.position += new Vector3(0, 0, 0.1f);
            float freezeSpeedMultiplier = 0.5f;
            float freezeRadiusMultiplier = ProjectileSize.Value;
            float freezeLifetime = ProjectileLifetime.Value;
            freezeOrbObject.GetComponent<FreezeZone>().Initialize(target.Transform.position, freezeSpeedMultiplier, freezeRadiusMultiplier, freezeLifetime);
            soundHandler.SoundShoot();
        }
        #endregion
    }
}
