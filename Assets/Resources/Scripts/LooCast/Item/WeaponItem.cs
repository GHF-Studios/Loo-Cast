using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Item
{
    using Data;
    using LooCast.Resource;
    using LooCast.Attribute.Stat;

    public class WeaponItem : UniqueItem
    {
        #region Data
        public WeaponItemData Data { get; private set; }
        #endregion

        #region Properties
        public GameObject MainGameObject
        {
            get
            {
                return mainGameObject;
            }
            
            protected set
            {
                mainGameObject = value;
                onMainGameObjectChanged.Invoke();
            }
        }
        public UnityEvent OnMainGameObjectChanged
        {
            get
            {
                return onMainGameObjectChanged;
            }
        }

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
        public float attackTimer { get; protected set; }
        public bool hasCooledDown { get; protected set; }
        #endregion

        #region Fields
        protected ITargeting targeting;
        protected GameSoundHandler soundHandler;

        private GameObject mainGameObject;
        private UnityEvent onMainGameObjectChanged;
        private Stats Stats;
        #endregion

        public WeaponItem(WeaponItemData data, GameObject mainGameObject, Stats stats) : base(data)
        {
            onMainGameObjectChanged = new UnityEvent();
            
            Data = data;

            targeting = GetComponent<ITargeting>();
            soundHandler = FindObjectOfType<GameSoundHandler>();

            damage = data.BaseDamage.Value * Stats.DamageMultiplier;
            critChance = data.BaseCritChance.Value * Stats.RandomChanceMultiplier;
            critDamage = data.BaseCritDamage.Value * Stats.DamageMultiplier;
            knockback = data.BaseKnockback.Value * Stats.KnockbackMultiplier;
            attackDelay = data.BaseAttackDelay.Value * Stats.AttackDelayMultiplier;
            projectileSpeed = data.BaseProjectileSpeed.Value * Stats.ProjectileSpeedMultiplier;
            projectileSize = data.BaseProjectileSize.Value * Stats.ProjectileSizeMultiplier;
            projectileLifetime = data.BaseProjectileLifetime.Value;
            piercing = data.BasePiercing.Value + Stats.PiercingIncrease;
            armorPenetration = data.BaseArmorPenetration.Value + Stats.ArmorPenetrationIncrease;

            projectilePrefab = data.ProjectilePrefab;
            attackTimer = 0.0f;
            hasCooledDown = false;

            MainGameObject = mainGameObject;
            Stats = stats;
        }
    }
}