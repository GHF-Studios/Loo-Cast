using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Linq;

namespace LooCast.Weapon
{
    using Data;
    using Target;
    using Targeting;
    using Projectile;
    using System;

    public sealed class MultiplexerWeapon : Weapon
    {
        public int maxTargets { get; private set; }
        public int maxFragments { get; private set; }
        public int fragmentArmorPenetration { get; private set; }
        public bool isTargetSeeking { get; private set; }
        public GameObject fragmentPrefab { get; private set; }

        public void Initialize(MultiplexerWeaponData data, ITargeting targeting)
        {
            Initialize(data, targeting);

            maxTargets = data.BaseMaxTargets.Value;
            maxFragments = data.BaseMaxFragments.Value;
            fragmentArmorPenetration = data.BaseFragmentArmorPenetration.Value;
            isTargetSeeking = data.IsTargetSeeking.Value;
            fragmentPrefab = data.FragmentPrefab;
        }

        public override bool TryFire()
        {
            if (attackTimer <= 0.0f && hasCooledDown)
            {
                List<Target> targets = AcquireTargets(maxTargets, TargetingMode.Closest);
                if (targets == null || targets.Count == 0)
                {
                    return false;
                }

                foreach (Target target in targets)
                {
                    GameObject bulletObject = Instantiate(projectilePrefab, transform.position, Quaternion.identity);
                    bulletObject.transform.position += new Vector3(0, 0, 0.1f);
                    var finalFragments = maxFragments;
                    if (maxFragments >= 1)
                    {
                        finalFragments = new Random().Next(1, maxFragments);
                    }
                    bulletObject.GetComponent<MultiplexerProjectile>().Initialize(target, gameObject, damage, critChance, critDamage, knockback, projectileSpeed, projectileSize, baseProjectileLifetime, piercing, armorPenetration, finalFragments, fragmentArmorPenetration, isTargetSeeking, fragmentPrefab);
                }
                soundHandler.SoundShoot();

                attackTimer = attackDelay;
                hasCooledDown = false;
                return true;
            }
            return false;
        }
    } 
}
