using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item
{
    using Data;
    using Attribute.Stat;
    using Target;
    using Projectile;
    using Util;

    public class ChargedPlasmaLauncherWeaponItem : WeaponItem
    {
        #region Data
        public ChargedPlasmaLauncherWeaponItemData ChargedPlasmaLauncherWeaponItemData { get; private set; }
        #endregion

        #region Properties
        public float ArcLifetime { get; private set; }
        public float ArcInitialWidth { get; private set; }
        public float ArcWidthMultiplier { get; private set; }
        public float ArcMinWidth { get; private set; }
        public int ArcBranchAttempts { get; private set; }
        public float MinSpreadDistance { get; private set; }
        public float MinSpreadDistanceMultiplier { get; private set; }
        public float MaxSpreadDistance { get; private set; }
        public float MaxSpreadDistanceMultiplier { get; private set; }
        public float MinSpreadAngle { get; private set; }
        public float MinSpreadAngleMultiplier { get; private set; }
        public float MaxSpreadAngle { get; private set; }
        public float MaxSpreadAngleMultiplier { get; private set; }
        public float SpreadChance { get; private set; }
        public float SpreadChanceMultiplier { get; private set; }
        public float MinBranchDistance { get; private set; }
        public float MinBranchDistanceMultiplier { get; private set; }
        public float MaxBranchDistance { get; private set; }
        public float MaxBranchDistanceMultiplier { get; private set; }
        public float MinBranchAngle { get; private set; }
        public float MinBranchAngleMultiplier { get; private set; }
        public float MaxBranchAngle { get; private set; }
        public float MaxBranchAngleMultiplier { get; private set; }
        public float BranchChance { get; private set; }
        public float BranchChanceMultiplier { get; private set; }
        public int MaxRecursionDepth { get; private set; }
        #endregion

        #region Fields
        #endregion

        #region Constructors
        public ChargedPlasmaLauncherWeaponItem(ChargedPlasmaLauncherWeaponItemData data, Stats stats, bool autoFire) : base(data, stats, autoFire)
        {
            ArcLifetime = data.ArcLifetime.Value;
            ArcInitialWidth = data.ArcInitialWidth.Value;
            ArcWidthMultiplier = data.ArcWidthMultiplier.Value;
            ArcMinWidth = data.ArcMinWidth.Value;
            ArcBranchAttempts = data.ArcBranchAttempts.Value;
            MinSpreadDistance = data.MinSpreadDistance.Value;
            MinSpreadDistanceMultiplier = data.MinSpreadDistanceMultiplier.Value;
            MaxSpreadDistance = data.MaxSpreadDistance.Value;
            MaxSpreadDistanceMultiplier = data.MaxSpreadDistanceMultiplier.Value;
            MinSpreadAngle = data.MinSpreadAngle.Value;
            MinSpreadAngleMultiplier = data.MinSpreadAngleMultiplier.Value;
            MaxSpreadAngle = data.MaxSpreadAngle.Value;
            MaxSpreadAngleMultiplier = data.MaxSpreadAngleMultiplier.Value;
            SpreadChance = data.SpreadChance.Value;
            SpreadChanceMultiplier = data.SpreadChanceMultiplier.Value;
            MinBranchDistance = data.MinBranchDistance.Value;
            MinBranchDistanceMultiplier = data.MinBranchDistanceMultiplier.Value;
            MaxBranchDistance = data.MaxBranchDistance.Value;
            MaxBranchDistanceMultiplier = data.MaxBranchDistanceMultiplier.Value;
            MinBranchAngle = data.MinBranchAngle.Value;
            MinBranchAngleMultiplier = data.MinBranchAngleMultiplier.Value;
            MaxBranchAngle = data.MaxBranchAngle.Value;
            MaxBranchAngleMultiplier = data.MaxBranchAngleMultiplier.Value;
            BranchChance = data.BranchChance.Value;
            BranchChanceMultiplier = data.BranchChanceMultiplier.Value;
            MaxRecursionDepth = data.MaxRecursionDepth.Value;
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
            bulletObject.GetComponent<ChargedPlasmaProjectile>().Initialize(target, ItemContainer.OriginObject, TeamUtil.GetTeam(ItemContainer.OriginObject.tag), Damage.Value, CritChance.Value, CritDamage.Value, Knockback.Value, ProjectileSpeed.Value, ProjectileSize.Value, ProjectileLifetime.Value, ArmorPenetration.Value, ArcLifetime, ArcInitialWidth, ArcWidthMultiplier, ArcMinWidth, ArcBranchAttempts, MinSpreadDistance, MinSpreadDistanceMultiplier, MaxSpreadDistance, MaxSpreadDistanceMultiplier, MinSpreadAngle, MinSpreadAngleMultiplier, MaxSpreadAngle, MaxSpreadAngleMultiplier, SpreadChance, SpreadChanceMultiplier, MinBranchDistance, MinBranchDistanceMultiplier, MaxBranchDistance, MaxBranchDistanceMultiplier, MinBranchAngle, MinBranchAngleMultiplier, MaxBranchAngle, MaxBranchAngleMultiplier, BranchChance, BranchChanceMultiplier, MaxRecursionDepth);
            soundHandler.SoundShoot();
        }
        #endregion
    }
}
