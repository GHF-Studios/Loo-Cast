using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Weapon
{
    using Data;
    using Projectile;
    using Target;
    using Targeting;

    public sealed class ChargedPlasmaLauncherWeapon : Weapon
    {
        public ChargedPlasmaLauncherWeaponData Data;

        public float arcLifetime { get; private set; }
        public float arcInitialWidth { get; private set; }
        public float arcWidthMultiplier { get; private set; }
        public float arcMinWidth { get; private set; }
        public int arcBranchAttempts { get; private set; }
        public float minSpreadDistance { get; private set; }
        public float minSpreadDistanceMultiplier { get; private set; }
        public float maxSpreadDistance { get; private set; }
        public float maxSpreadDistanceMultiplier { get; private set; }
        public float minSpreadAngle { get; private set; }
        public float minSpreadAngleMultiplier { get; private set; }
        public float maxSpreadAngle { get; private set; }
        public float maxSpreadAngleMultiplier { get; private set; }
        public float spreadChance { get; private set; }
        public float spreadChanceMultiplier { get; private set; }
        public float minBranchDistance { get; private set; }
        public float minBranchDistanceMultiplier { get; private set; }
        public float maxBranchDistance { get; private set; }
        public float maxBranchDistanceMultiplier { get; private set; }
        public float minBranchAngle { get; private set; }
        public float minBranchAngleMultiplier { get; private set; }
        public float maxBranchAngle { get; private set; }
        public float maxBranchAngleMultiplier { get; private set; }
        public float branchChance { get; private set; }
        public float branchChanceMultiplier { get; private set; }
        public int maxRecursionDepth { get; private set; }

        private void Start()
        {
            Initialize(Data);

            arcLifetime = Data.ArcLifetime.Value;
            arcInitialWidth = Data.ArcInitialWidth.Value;
            arcWidthMultiplier = Data.ArcWidthMultiplier.Value;
            arcMinWidth = Data.ArcMinWidth.Value;
            arcBranchAttempts = Data.ArcBranchAttempts.Value;
            minSpreadDistance = Data.MinSpreadDistance.Value;
            minSpreadDistanceMultiplier = Data.MinSpreadDistanceMultiplier.Value;
            maxSpreadDistance = Data.MaxSpreadDistance.Value;
            maxSpreadDistanceMultiplier = Data.MaxSpreadDistanceMultiplier.Value;
            minSpreadAngle = Data.MinSpreadAngle.Value;
            minSpreadAngleMultiplier = Data.MinSpreadAngleMultiplier.Value;
            maxSpreadAngle = Data.MaxSpreadAngle.Value;
            maxSpreadAngleMultiplier = Data.MaxSpreadAngleMultiplier.Value;
            spreadChance = Data.SpreadChance.Value;
            spreadChanceMultiplier = Data.SpreadChanceMultiplier.Value;
            minBranchDistance = Data.MinBranchDistance.Value;
            minBranchDistanceMultiplier = Data.MinBranchDistanceMultiplier.Value;
            maxBranchDistance = Data.MaxBranchDistance.Value;
            maxBranchDistanceMultiplier = Data.MaxBranchDistanceMultiplier.Value;
            minBranchAngle = Data.MinBranchAngle.Value;
            minBranchAngleMultiplier = Data.MinBranchAngleMultiplier.Value;
            maxBranchAngle = Data.MaxBranchAngle.Value;
            maxBranchAngleMultiplier = Data.MaxBranchAngleMultiplier.Value;
            branchChance = Data.BranchChance.Value;
            branchChanceMultiplier = Data.BranchChanceMultiplier.Value;
            maxRecursionDepth = Data.MaxRecursionDepth.Value;
        }

        public override bool TryFire()
        {
            if (attackTimer <= 0.0f && hasCooledDown)
            {
                List<Target> targets = AcquireTargets(1, TargetingMode.Closest);
                if (targets == null || targets.Count == 0)
                {
                    return false;
                }
                Target target = targets[0];

                GameObject bulletObject = Instantiate(projectilePrefab, transform.position, Quaternion.identity);
                bulletObject.transform.position += new Vector3(0, 0, 0.1f);
                bulletObject.GetComponent<ChargedPlasmaProjectile>().Initialize(target, gameObject, damage, critChance, critDamage, knockback, projectileSpeed, projectileSize, baseProjectileLifetime, armorPenetration, arcLifetime, arcInitialWidth, arcWidthMultiplier, arcMinWidth, arcBranchAttempts, minSpreadDistance, minSpreadDistanceMultiplier, maxSpreadDistance, maxSpreadDistanceMultiplier, minSpreadAngle, minSpreadAngleMultiplier, maxSpreadAngle, maxSpreadAngleMultiplier, spreadChance, spreadChanceMultiplier, minBranchDistance, minBranchDistanceMultiplier, maxBranchDistance, maxBranchDistanceMultiplier, minBranchAngle, minBranchAngleMultiplier, maxBranchAngle, maxBranchAngleMultiplier, branchChance, branchChanceMultiplier, maxRecursionDepth);
                soundHandler.SoundShoot();

                attackTimer = attackDelay;
                hasCooledDown = false;
                return true;
            }
            return false;
        }
    } 
}
