using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Weapon.Data
{
    using LooCast.Data;

    public abstract class WeaponData : ScriptableObject
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
    } 
}
