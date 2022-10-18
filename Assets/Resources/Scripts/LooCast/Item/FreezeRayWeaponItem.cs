﻿using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item
{
    using Data;
    using LooCast.Attribute.Stat;
    using LooCast.Targeting;
    using LooCast.Target;
    using LooCast.AOE;
    using LooCast.Util;

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

        }
        #endregion

        #region Methods
        public override void Fire()
        {
            NewTarget[] targets = TargetingUtil.GetClosestTargets(ItemContainer.OriginObject.transform.position, Range, TagUtil.GetEnemyTags(ItemContainer.OriginObject));
            if (targets == null || targets.Length == 0)
            {
                return;
            }
            NewTarget target = targets[0];

            GameObject freezeOrbObject = GameObject.Instantiate(ProjectilePrefab, ItemContainer.OriginObject.transform.position, Quaternion.identity);
            freezeOrbObject.transform.position += new Vector3(0, 0, 0.1f);
            float freezeSpeedMultiplier = 0.5f;
            float freezeRadiusMultiplier = ProjectileSize;
            float freezeLifetime = ProjectileLifetime;
            freezeOrbObject.GetComponent<FreezeZone>().Initialize(target.Transform.position, freezeSpeedMultiplier, freezeRadiusMultiplier, freezeLifetime);
            soundHandler.SoundShoot();
        }
        #endregion
    }
}
