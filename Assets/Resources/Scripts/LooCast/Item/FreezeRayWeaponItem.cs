using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item
{
    using Data;
    using LooCast.Attribute.Stat;
    using LooCast.Targeting;
    using LooCast.Target;
    using LooCast.AOE;

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
        public FreezeRayWeaponItem(FreezeRayWeaponItemData data, Stats stats, ITargeting mainTargeting, GameObject originObject) : base(data, stats, mainTargeting, originObject)
        {

        }
        #endregion

        #region Methods
        public override bool TryFire()
        {
            if (canFire)
            {
                canFire = false;
                fireTimer.Start();

                List<Target> targets = AcquireTargets(1, TargetingMode.Closest);
                if (targets == null || targets.Count == 0)
                {
                    return false;
                }
                Target target = targets[0];

                GameObject freezeOrbObject = GameObject.Instantiate(projectilePrefab, originObject.transform.position, Quaternion.identity);
                freezeOrbObject.transform.position += new Vector3(0, 0, 0.1f);
                float freezeSpeedMultiplier = 0.5f;
                float freezeRadiusMultiplier = projectileSize;
                float freezeLifetime = projectileLifetime;
                freezeOrbObject.GetComponent<FreezeZone>().Initialize(target.transform.position, freezeSpeedMultiplier, freezeRadiusMultiplier, freezeLifetime);
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
