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
    using LooCast.Variable;

    public abstract class WeaponItem : UniqueItem
    {
        #region Data
        public WeaponItemData WeaponItemData { get; private set; }
        #endregion

        #region Properties
        public FloatComputedVariable Damage { get; protected set; }
        public FloatComputedVariable CritChance { get; protected set; }
        public FloatComputedVariable CritDamage { get; protected set; }
        public FloatComputedVariable Knockback { get; protected set; }
        public FloatComputedVariable AttackDelay { get; protected set; }
        public FloatComputedVariable ProjectileSpeed { get; protected set; }
        public FloatComputedVariable ProjectileSize { get; protected set; }
        public FloatComputedVariable ProjectileLifetime { get; protected set; }
        public IntComputedVariable Piercing { get; protected set; }
        public IntComputedVariable ArmorPenetration { get; protected set; }
        public GameObject ProjectilePrefab { get; protected set; }
        public bool AutoFire { get; protected set; }
        public FloatComputedVariable Range { get; protected set; }
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
            AutoFire = autoFire;
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

            Damage = new FloatComputedVariable(data.BaseDamage.Value);
            Damage.AddPermanentMultiplier(stats.DamageMultiplier);
            CritChance = new FloatComputedVariable(data.BaseCritChance.Value);
            CritChance.AddPermanentMultiplier(stats.RandomChanceMultiplier);
            CritDamage = new FloatComputedVariable(data.BaseCritDamage.Value);
            CritDamage.AddPermanentMultiplier(stats.DamageMultiplier);
            Knockback = new FloatComputedVariable(data.BaseKnockback.Value);
            Knockback.AddPermanentMultiplier(stats.KnockbackMultiplier);
            AttackDelay = new FloatComputedVariable(data.BaseAttackDelay.Value);
            AttackDelay.AddPermanentMultiplier(stats.AttackDelayMultiplier);
            ProjectileSpeed = new FloatComputedVariable(data.BaseProjectileSpeed.Value);
            ProjectileSpeed.AddPermanentMultiplier(stats.ProjectileSpeedMultiplier);
            ProjectileSize = new FloatComputedVariable(data.BaseProjectileSize.Value);
            ProjectileSize.AddPermanentMultiplier(stats.ProjectileSizeMultiplier);
            ProjectileLifetime = new FloatComputedVariable(data.BaseProjectileLifetime.Value);
            Piercing = new IntComputedVariable(data.BasePiercing.Value);
            Piercing.AddPermanentIncrease(stats.PiercingIncrease);
            ArmorPenetration = new IntComputedVariable(data.BaseArmorPenetration.Value);
            ArmorPenetration.AddPermanentIncrease(stats.ArmorPenetrationIncrease);
            ProjectilePrefab = data.ProjectilePrefab;
            Range = new FloatComputedVariable(data.BaseRange.Value);
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