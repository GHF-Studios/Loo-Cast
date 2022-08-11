using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Health.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "EnemyHealthData", menuName = "Data/Health/EnemyHealthData", order = 0)]
    public class EnemyHealthData : ScriptableObject
    {
        public FloatDataReference BaseMaxHealth;
        public FloatDataReference BaseRegenerationAmount;
        public FloatDataReference BaseRegenerationTime;
        public IntDataReference BaseDefense;
        public GameObject DamageIndicatorPrefab;
        public FloatDataReference BaseExperienceDropChance;
        public FloatDataReference BaseMagnetDropChance;
        public FloatDataReference BaseExperienceDropAmount;
        public GameObject ExperienceOrbPrefab;
        public GameObject MagnetOrbPrefab;
    } 
}
