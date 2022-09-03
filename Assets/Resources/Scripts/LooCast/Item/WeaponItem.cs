using System;
using System.Timers;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Item
{
    using Data;
    using LooCast.Targeting;
    using LooCast.Attribute.Stat;
    using LooCast.Sound;
    using LooCast.Target;

    public abstract class WeaponItem : UniqueItem
    {
        #region Data
        public WeaponItemData WeaponItemData { get; private set; }
        #endregion

        #region Properties
        public float damage { get; protected set; }
        public float critChance { get; protected set; }
        public float critDamage { get; protected set; }
        public float knockback { get; protected set; }
        public float attackDelay { get; protected set; }
        public float projectileSpeed { get; protected set; }
        public float projectileSize { get; protected set; }
        public float projectileLifetime { get; protected set; }
        public int piercing { get; protected set; }
        public int armorPenetration { get; protected set; }
        public GameObject projectilePrefab { get; protected set; }
        public bool autoFire { get; protected set; }
        #endregion

        #region Fields
        protected Stats stats;
        protected ITargeting mainTargeting;
        protected GameSoundHandler soundHandler;
        protected Timer fireTimer;
        protected bool canFire;
        protected GameObject originObject;
        #endregion

        #region Constructors
        public WeaponItem(WeaponItemData data, ItemObject itemObject, Stats stats, bool autoFire = false) : base(data, itemObject)
        {
            WeaponItemData = data;

            this.stats = stats;
            this.autoFire = autoFire;
            soundHandler = GameObject.FindObjectOfType<GameSoundHandler>();
            fireTimer = new Timer(data.BaseAttackDelay.Value * 1000);
            fireTimer.Elapsed += (sender, elapsedEventArgs) =>
            {
                canFire = true;
                if (autoFire)
                {
                    TryFire();
                }
            };
            if (!IsDropped)
            {
                fireTimer.Start();
            }
            canFire = false;
            OnSpawn.AddListener(() => 
            { 
                fireTimer.Stop(); 
                canFire = false; 
                mainTargeting = null; 
                originObject = null; 
            });
            OnPickup.AddListener((itemContainer) => 
            {
                if (!itemContainer.IsBoundToObject())
                {
                    return;
                }
                fireTimer.Start(); 
                canFire = true; 
                mainTargeting = itemContainer.OriginObject.GetComponentInChildren<ITargeting>();
                if (mainTargeting == null)
                {
                    throw new NullReferenceException("No Targeting found in origin!");
                }
                originObject = itemContainer.OriginObject; 
            });

            damage = data.BaseDamage.Value * this.stats.DamageMultiplier;
            critChance = data.BaseCritChance.Value * this.stats.RandomChanceMultiplier;
            critDamage = data.BaseCritDamage.Value * this.stats.DamageMultiplier;
            knockback = data.BaseKnockback.Value * this.stats.KnockbackMultiplier;
            attackDelay = data.BaseAttackDelay.Value * this.stats.AttackDelayMultiplier;
            projectileSpeed = data.BaseProjectileSpeed.Value * this.stats.ProjectileSpeedMultiplier;
            projectileSize = data.BaseProjectileSize.Value * this.stats.ProjectileSizeMultiplier;
            projectileLifetime = data.BaseProjectileLifetime.Value;
            piercing = data.BasePiercing.Value + this.stats.PiercingIncrease;
            armorPenetration = data.BaseArmorPenetration.Value + this.stats.ArmorPenetrationIncrease;
            projectilePrefab = data.ProjectilePrefab;
        }
        #endregion

        #region Methods
        public bool TryFire()
        {
            if (canFire && !IsDropped)
            {
                canFire = false;
                Fire();
                fireTimer.Start();
                return true;
            }
            return false;
        }

        public abstract void Fire();

        public override void Use()
        {
            TryFire();
        }

        protected virtual List<Target> AcquireTargets(int count, TargetingMode targetType)
        {
            List<Target> targetsFound;

            switch (targetType)
            {
                case TargetingMode.Closest:
                    targetsFound = mainTargeting.ClosestTargets;
                    break;
                case TargetingMode.Furthest:
                    targetsFound = mainTargeting.FurthestTargets;
                    break;
                case TargetingMode.Random:
                    targetsFound = mainTargeting.RandomTargets;
                    break;
                case TargetingMode.RandomOnscreen:
                    targetsFound = mainTargeting.RandomOnscreenTargets;
                    break;
                case TargetingMode.RandomProximity:
                    targetsFound = mainTargeting.RandomProximityTargets;
                    break;
                default:
                    targetsFound = null;
                    break;
            }

            if (targetsFound == null)
            {
                return null;
            }

            List<Target> targets = new List<Target>();

            foreach (Target target in targetsFound)
            {
                if (targets.Count >= count)
                {
                    break;
                }

                if (target == null || !target.IsValid() || target.IsLocked())
                {
                    continue;
                }

                Target.EngageTargetLock(target, out bool targetLockSuccess);

                if (targetLockSuccess)
                {
                    targets.Add(target);
                }
                else
                {
                    continue;
                }
            }

            return targets;
        }
        #endregion
    }
}