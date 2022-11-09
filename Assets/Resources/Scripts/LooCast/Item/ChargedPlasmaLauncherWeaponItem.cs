using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item
{
    using Data;
    using Attribute.Stat;
    using Target;
    using Projectile;
    using Util;
    using Variable;

    public class ChargedPlasmaLauncherWeaponItem : WeaponItem
    {
        #region Data
        public ChargedPlasmaLauncherWeaponItemData ChargedPlasmaLauncherWeaponItemData { get; private set; }
        #endregion

        #region Properties
        public FloatComputedVariable ArcLifetime { get; private set; }
        public FloatComputedVariable ArcInitialWidth { get; private set; }
        public FloatComputedVariable ArcWidthMultiplier { get; private set; }
        public FloatComputedVariable ArcMinWidth { get; private set; }
        public IntComputedVariable ArcBranchAttempts { get; private set; }
        public FloatComputedVariable MinSpreadDistance { get; private set; }
        public FloatComputedVariable MinSpreadDistanceMultiplier { get; private set; }
        public FloatComputedVariable MaxSpreadDistance { get; private set; }
        public FloatComputedVariable MaxSpreadDistanceMultiplier { get; private set; }
        public FloatComputedVariable MinSpreadAngle { get; private set; }
        public FloatComputedVariable MinSpreadAngleMultiplier { get; private set; }
        public FloatComputedVariable MaxSpreadAngle { get; private set; }
        public FloatComputedVariable MaxSpreadAngleMultiplier { get; private set; }
        public FloatComputedVariable SpreadChance { get; private set; }
        public FloatComputedVariable SpreadChanceMultiplier { get; private set; }
        public FloatComputedVariable MinBranchDistance { get; private set; }
        public FloatComputedVariable MinBranchDistanceMultiplier { get; private set; }
        public FloatComputedVariable MaxBranchDistance { get; private set; }
        public FloatComputedVariable MaxBranchDistanceMultiplier { get; private set; }
        public FloatComputedVariable MinBranchAngle { get; private set; }
        public FloatComputedVariable MinBranchAngleMultiplier { get; private set; }
        public FloatComputedVariable MaxBranchAngle { get; private set; }
        public FloatComputedVariable MaxBranchAngleMultiplier { get; private set; }
        public FloatComputedVariable BranchChance { get; private set; }
        public FloatComputedVariable BranchChanceMultiplier { get; private set; }
        public IntComputedVariable MaxRecursionDepth { get; private set; }
        #endregion

        #region Fields
        #endregion

        #region Constructors
        public ChargedPlasmaLauncherWeaponItem(ChargedPlasmaLauncherWeaponItemData data, Stats stats, bool autoFire) : base(data, stats, autoFire)
        {
            ChargedPlasmaLauncherWeaponItemData = data;

            ArcLifetime = new FloatComputedVariable(data.ArcLifetime.Value);
            ArcInitialWidth = new FloatComputedVariable(data.ArcInitialWidth.Value);
            ArcWidthMultiplier = new FloatComputedVariable(data.ArcWidthMultiplier.Value);
            ArcMinWidth = new FloatComputedVariable(data.ArcMinWidth.Value);
            ArcBranchAttempts = new IntComputedVariable(data.ArcBranchAttempts.Value);
            MinSpreadDistance = new FloatComputedVariable(data.MinSpreadDistance.Value);
            MinSpreadDistanceMultiplier = new FloatComputedVariable(data.MinSpreadDistanceMultiplier.Value);
            MaxSpreadDistance = new FloatComputedVariable(data.MaxSpreadDistance.Value);
            MaxSpreadDistanceMultiplier = new FloatComputedVariable(data.MaxSpreadDistanceMultiplier.Value);
            MinSpreadAngle = new FloatComputedVariable(data.MinSpreadAngle.Value);
            MinSpreadAngleMultiplier = new FloatComputedVariable(data.MinSpreadAngleMultiplier.Value);
            MaxSpreadAngle = new FloatComputedVariable(data.MaxSpreadAngle.Value);
            MaxSpreadAngleMultiplier = new FloatComputedVariable(data.MaxSpreadAngleMultiplier.Value);
            SpreadChance = new FloatComputedVariable(data.SpreadChance.Value);
            SpreadChanceMultiplier = new FloatComputedVariable(data.SpreadChanceMultiplier.Value);
            MinBranchDistance = new FloatComputedVariable(data.MinBranchDistance.Value);
            MinBranchDistanceMultiplier = new FloatComputedVariable(data.MinBranchDistanceMultiplier.Value);
            MaxBranchDistance = new FloatComputedVariable(data.MaxBranchDistance.Value);
            MaxBranchDistanceMultiplier = new FloatComputedVariable(data.MaxBranchDistanceMultiplier.Value);
            MinBranchAngle = new FloatComputedVariable(data.MinBranchAngle.Value);
            MinBranchAngleMultiplier = new FloatComputedVariable(data.MinBranchAngleMultiplier.Value);
            MaxBranchAngle = new FloatComputedVariable(data.MaxBranchAngle.Value);
            MaxBranchAngleMultiplier = new FloatComputedVariable(data.MaxBranchAngleMultiplier.Value);
            BranchChance = new FloatComputedVariable(data.BranchChance.Value);
            BranchChanceMultiplier = new FloatComputedVariable(data.BranchChanceMultiplier.Value);
            MaxRecursionDepth = new IntComputedVariable(data.MaxRecursionDepth.Value);
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
            bulletObject.GetComponent<ChargedPlasmaProjectile>().Initialize(target, ItemContainer.OriginObject, TeamUtil.GetTeam(ItemContainer.OriginObject.tag), Damage.Value, CritChance.Value, CritDamage.Value, Knockback.Value, ProjectileSpeed.Value, ProjectileSize.Value, ProjectileLifetime.Value, ArmorPenetration.Value, ArcLifetime.Value, ArcInitialWidth.Value, ArcWidthMultiplier.Value, ArcMinWidth.Value, ArcBranchAttempts.Value, MinSpreadDistance.Value, MinSpreadDistanceMultiplier.Value, MaxSpreadDistance.Value, MaxSpreadDistanceMultiplier.Value, MinSpreadAngle.Value, MinSpreadAngleMultiplier.Value, MaxSpreadAngle.Value, MaxSpreadAngleMultiplier.Value, SpreadChance.Value, SpreadChanceMultiplier.Value, MinBranchDistance.Value, MinBranchDistanceMultiplier.Value, MaxBranchDistance.Value, MaxBranchDistanceMultiplier.Value, MinBranchAngle.Value, MinBranchAngleMultiplier.Value, MaxBranchAngle.Value, MaxBranchAngleMultiplier.Value, BranchChance.Value, BranchChanceMultiplier.Value, MaxRecursionDepth.Value);
            soundHandler.SoundShoot();
        }
        #endregion
    }
}
