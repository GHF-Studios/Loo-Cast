using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item
{
    using Data;
    using LooCast.Attribute.Stat;
    using LooCast.Targeting;
    using LooCast.Target;
    using LooCast.Projectile;

    public class MultiplexerWeaponItem : WeaponItem
    {
        #region Data
        public MultiplexerWeaponItemData MultiplexerWeaponItemData { get; private set; }
        #endregion

        #region Properties
        public int maxTargets { get; private set; }
        public int maxFragments { get; private set; }
        public int fragmentArmorPenetration { get; private set; }
        public bool isTargetSeeking { get; private set; }
        public GameObject fragmentPrefab { get; private set; }
        #endregion

        #region Fields
        #endregion

        #region Constructors
        public MultiplexerWeaponItem(MultiplexerWeaponItemData data, Stats stats, ITargeting mainTargeting, GameObject originObject) : base(data, stats, mainTargeting, originObject)
        {
            maxTargets = data.BaseMaxTargets.Value;
            maxFragments = data.BaseMaxFragments.Value;
            fragmentArmorPenetration = data.BaseFragmentArmorPenetration.Value;
            isTargetSeeking = data.IsTargetSeeking.Value;
            fragmentPrefab = data.FragmentPrefab;
        }
        #endregion

        #region Methods
        public override bool TryFire()
        {
            if (canFire)
            {
                canFire = false;
                fireTimer.Start();

                List<Target> targets = AcquireTargets(maxTargets, TargetingMode.Closest);
                if (targets == null || targets.Count == 0)
                {
                    return false;
                }

                foreach (Target target in targets)
                {
                    GameObject bulletObject = GameObject.Instantiate(projectilePrefab, originObject.transform.position, Quaternion.identity);
                    bulletObject.transform.position += new Vector3(0, 0, 0.1f);
                    var finalFragments = maxFragments;
                    if (maxFragments >= 1)
                    {
                        finalFragments = UnityEngine.Random.Range(1, maxFragments);
                    }
                    bulletObject.GetComponent<MultiplexerProjectile>().Initialize(target, originObject, damage, critChance, critDamage, knockback, projectileSpeed, projectileSize, projectileLifetime, piercing, armorPenetration, finalFragments, fragmentArmorPenetration, isTargetSeeking, fragmentPrefab);
                }
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
