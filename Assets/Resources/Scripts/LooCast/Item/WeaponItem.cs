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

    public abstract class WeaponItem : UniqueItem, IUpgradableItem
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
        public FloatComputedVariable Range { get; protected set; }
        public GameObject ProjectilePrefab { get; protected set; }
        public bool AutoFire { get; protected set; }
        #endregion

        #region Fields
        protected Stats stats;
        protected GameSoundHandler soundHandler;
        protected Timer fireTimer;
        protected bool canFire;

        #region IUpgradableItem
        private Dictionary<int, Action> statUpgradeSetRemoveActionDictionary = new Dictionary<int, Action>();
        #endregion

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

            Damage = new FloatComputedVariable(data.BaseDamage.Value);
            CritChance = new FloatComputedVariable(data.BaseCritChance.Value);
            CritDamage = new FloatComputedVariable(data.BaseCritDamage.Value);
            Knockback = new FloatComputedVariable(data.BaseKnockback.Value);
            AttackDelay = new FloatComputedVariable(data.BaseAttackDelay.Value);
            ProjectileSpeed = new FloatComputedVariable(data.BaseProjectileSpeed.Value);
            ProjectileSize = new FloatComputedVariable(data.BaseProjectileSize.Value);
            ProjectileLifetime = new FloatComputedVariable(data.BaseProjectileLifetime.Value);
            Piercing = new IntComputedVariable(data.BasePiercing.Value);
            ArmorPenetration = new IntComputedVariable(data.BaseArmorPenetration.Value);
            Range = new FloatComputedVariable(data.BaseRange.Value);
            ProjectilePrefab = data.ProjectilePrefab;
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

        #region IUpgradableItem
        public void ApplyItemStatUpgradeSet(int upgradeSetID, Stats stats)
        {
            if (statUpgradeSetRemoveActionDictionary.ContainsKey(upgradeSetID))
            {
                return;
            }

            Multiplier damageMultiplier = Damage.AddPermanentMultiplier(stats.DamageMultiplier);
            Multiplier critChanceMultiplier = CritChance.AddPermanentMultiplier(stats.RandomChanceMultiplier);
            Multiplier critDamageMultiplier = CritDamage.AddPermanentMultiplier(stats.DamageMultiplier);
            Multiplier knockbackMultiplier = Knockback.AddPermanentMultiplier(stats.KnockbackMultiplier);
            Multiplier attackDelayMultiplier = AttackDelay.AddPermanentMultiplier(stats.AttackDelayMultiplier);
            Multiplier projectileSpeedMultiplier = ProjectileSpeed.AddPermanentMultiplier(stats.ProjectileSpeedMultiplier);
            Multiplier projectileSizeMultiplier = ProjectileSize.AddPermanentMultiplier(stats.ProjectileSizeMultiplier);
            Multiplier projectileLifetimeMultiplier = ProjectileLifetime.AddPermanentMultiplier(stats.DamageMultiplier);
            Increase piercingIncrease = Piercing.AddPermanentIncrease(stats.PiercingIncrease);
            Increase armorPenetrationIncrease = ArmorPenetration.AddPermanentIncrease(stats.ArmorPenetrationIncrease);
            Multiplier rangeMultiplier = Range.AddPermanentMultiplier(stats.RangeMultiplier);

            statUpgradeSetRemoveActionDictionary.Add(upgradeSetID, () =>
            {
                Damage.RemovePermanentMultiplier(damageMultiplier);
                CritChance.RemovePermanentMultiplier(critChanceMultiplier);
                CritDamage.RemovePermanentMultiplier(critDamageMultiplier);
                Knockback.RemovePermanentMultiplier(knockbackMultiplier);
                AttackDelay.RemovePermanentMultiplier(attackDelayMultiplier);
                ProjectileSpeed.RemovePermanentMultiplier(projectileSpeedMultiplier);
                ProjectileSize.RemovePermanentMultiplier(projectileSizeMultiplier);
                ProjectileLifetime.RemovePermanentMultiplier(projectileLifetimeMultiplier);
                Piercing.RemovePermanentIncrease(piercingIncrease);
                ArmorPenetration.RemovePermanentIncrease(armorPenetrationIncrease);
                Range.RemovePermanentMultiplier(rangeMultiplier);
            });
        }

        public void RemoveItemStatUpgradeSet(int upgradeSetID)
        {
            if (statUpgradeSetRemoveActionDictionary.ContainsKey(upgradeSetID))
            {
                statUpgradeSetRemoveActionDictionary.TryGetValue(upgradeSetID, out Action statDegradeAction);
                statDegradeAction.Invoke();
                statUpgradeSetRemoveActionDictionary.Remove(upgradeSetID);
            }
        }
        #endregion

        #endregion
    }
}