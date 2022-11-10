using System;
using UnityEngine;

namespace LooCast.Item
{
    using Data;
    using Attribute.Stat;
    using Sound;
    using Util;
    using Variable;

    public abstract class WeaponItem : UpgradableItem
    {
        #region Data
        public WeaponItemData WeaponItemData { get; private set; }
        #endregion

        #region Properties
        public WeaponItemObject WeaponItemObject { get; private set; }
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
        protected TimerUtil.Timer fireTimer;
        protected bool canFire = false;
        #endregion

        #region Constructors
        public WeaponItem(WeaponItemData data, Stats stats, bool autoFire = false) : base(data)
        {
            WeaponItemData = data;

            this.stats = stats;
            AutoFire = autoFire;
            soundHandler = GameObject.FindObjectOfType<GameSoundHandler>();

            fireTimer = TimerUtil.CreateTimer(data.BaseAttackDelay.Value, false, autoFire);
            fireTimer.AddElapsedAction(() =>
            {
                canFire = true;
                TryFire();
            });
            fireTimer.Start();



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
            if (!canFire)
            {
                return false;
            }

            bool fireSuccess = Fire();
            if (!fireSuccess)
            {
                return false;
            }
            else
            {
                canFire = false;
                fireTimer.Start();
                return true;
            }
        }

        public abstract bool Fire();

        public override void Use()
        {
            TryFire();
        }

        public override void DropItem(Vector3 spawnPosition)
        {
            base.DropItem(spawnPosition);
            WeaponItemObject = (WeaponItemObject)ItemObject;
            if (WeaponItemObject == null)
            {
                throw new Exception("ItemObjectPrefab must contain a WeaponItemObject-component!");
            }
        }

        public override void ApplyItemStatUpgradeSet(int upgradeSetID, UpgradeSet upgradeSet)
        {
            if (upgradeSetRemovementActions.ContainsKey(upgradeSetID))
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

            upgradeSetRemovementActions.Add(upgradeSetID, () =>
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
        #endregion
    }
}