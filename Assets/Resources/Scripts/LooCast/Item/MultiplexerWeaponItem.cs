﻿using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item
{
    using Data;
    using Attribute.Stat;
    using Target;
    using Projectile;
    using Util;

    public class MultiplexerWeaponItem : WeaponItem
    {
        #region Data
        public MultiplexerWeaponItemData MultiplexerWeaponItemData { get; private set; }
        #endregion

        #region Properties
        public int MaxTargets { get; private set; }
        public int MaxFragments { get; private set; }
        public int FragmentArmorPenetration { get; private set; }
        public bool IsTargetSeeking { get; private set; }
        public GameObject FragmentPrefab { get; private set; }
        #endregion

        #region Fields
        #endregion

        #region Constructors
        public MultiplexerWeaponItem(MultiplexerWeaponItemData data, Stats stats, bool autoFire) : base(data, stats, autoFire)
        {
            MaxTargets = data.BaseMaxTargets.Value;
            MaxFragments = data.BaseMaxFragments.Value;
            FragmentArmorPenetration = data.BaseFragmentArmorPenetration.Value;
            IsTargetSeeking = data.IsTargetSeeking.Value;
            FragmentPrefab = data.FragmentPrefab;
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

            foreach (Target target in targets)
            {
                GameObject bulletObject = GameObject.Instantiate(ProjectilePrefab, ItemContainer.OriginObject.transform.position, Quaternion.identity);
                bulletObject.transform.position += new Vector3(0, 0, 0.1f);
                var finalFragments = MaxFragments;
                if (MaxFragments >= 1)
                {
                    finalFragments = UnityEngine.Random.Range(1, MaxFragments);
                }
                bulletObject.GetComponent<MultiplexerProjectile>().Initialize(target, ItemContainer.OriginObject, TeamUtil.GetTeam(ItemContainer.OriginObject.tag), Damage.Value, CritChance.Value, CritDamage.Value, Knockback.Value, ProjectileSpeed.Value, ProjectileSize.Value, ProjectileLifetime.Value, Piercing.Value, ArmorPenetration.Value, finalFragments, FragmentArmorPenetration, IsTargetSeeking, FragmentPrefab);
            }
            soundHandler.SoundShoot();
        }
        #endregion
    }
}
