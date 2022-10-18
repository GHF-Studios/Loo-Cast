using UnityEngine;

namespace LooCast.Item.Data
{
    using LooCast.Data;
    using LooCast.Attribute.Stat;

    public abstract class WeaponItemData : UniqueItemData
    {
        public FloatDataReference BaseDamage;
        public FloatDataReference BaseCritChance;
        public FloatDataReference BaseCritDamage;
        public FloatDataReference BaseKnockback;
        public FloatDataReference BaseAttackDelay;
        public FloatDataReference BaseProjectileSpeed;
        public FloatDataReference BaseProjectileSize;
        public FloatDataReference BaseProjectileLifetime;
        public IntDataReference BasePiercing;
        public IntDataReference BaseArmorPenetration;
        public GameObject ProjectilePrefab;
        public BoolDataReference AutoFire;
        public Stats Stats;
        public FloatDataReference Range;
    }
}