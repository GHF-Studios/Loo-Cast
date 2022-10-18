using System;
using System.Timers;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Item
{
    using Data;
    using LooCast.Attribute.Stat;
    using LooCast.Sound;

    public abstract class WeaponItem : UniqueItem
    {
        #region Data
        public WeaponItemData WeaponItemData { get; private set; }
        #endregion

        #region Properties
        public float Damage { get; protected set; }
        public float CritChance { get; protected set; }
        public float CritDamage { get; protected set; }
        public float Knockback { get; protected set; }
        public float AttackDelay { get; protected set; }
        public float ProjectileSpeed { get; protected set; }
        public float ProjectileSize { get; protected set; }
        public float ProjectileLifetime { get; protected set; }
        public int Piercing { get; protected set; }
        public int ArmorPenetration { get; protected set; }
        public GameObject ProjectilePrefab { get; protected set; }
        public bool AutoFire { get; protected set; }
        public float Range { get; protected set; }
        #endregion

        #region Fields
        protected Stats stats;
        protected GameSoundHandler soundHandler;
        protected Timer fireTimer;
        protected bool canFire;
        #endregion

        #region Constructors
        public WeaponItem(WeaponItemData data, Stats stats, bool autoFire = false) : base(data)
        {
            WeaponItemData = data;

            this.stats = stats;
            this.AutoFire = autoFire;
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
            canFire = false;

            OnContainmentStateChange.AddListener(() =>
            {
                switch (ItemContainmentState)
                {
                    case ContainmentState.Contained:
                        if (ItemContainer.IsBoundToObject())
                        {
                            fireTimer.Start();
                            canFire = true;
                        }
                        break;
                    case ContainmentState.Dropped:
                        fireTimer.Stop();
                        canFire = false;
                        break;
                    case ContainmentState.Standalone:
                        fireTimer.Stop();
                        canFire = false;
                        break;
                    default:
                        break;
                }
            });

            Damage = data.BaseDamage.Value * stats.DamageMultiplier;
            CritChance = data.BaseCritChance.Value * stats.RandomChanceMultiplier;
            CritDamage = data.BaseCritDamage.Value * stats.DamageMultiplier;
            Knockback = data.BaseKnockback.Value * stats.KnockbackMultiplier;
            AttackDelay = data.BaseAttackDelay.Value * stats.AttackDelayMultiplier;
            ProjectileSpeed = data.BaseProjectileSpeed.Value * stats.ProjectileSpeedMultiplier;
            ProjectileSize = data.BaseProjectileSize.Value * stats.ProjectileSizeMultiplier;
            ProjectileLifetime = data.BaseProjectileLifetime.Value;
            Piercing = data.BasePiercing.Value + stats.PiercingIncrease;
            ArmorPenetration = data.BaseArmorPenetration.Value + stats.ArmorPenetrationIncrease;
            ProjectilePrefab = data.ProjectilePrefab;
            Range = data.Range.Value;
        }
        #endregion

        #region Methods
        public bool TryFire()
        {
            if (canFire)
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
        #endregion
    }
}