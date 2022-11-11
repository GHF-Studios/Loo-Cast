using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Health.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "AllyHealthData", menuName = "Data/Health/AllyHealthData", order = 0)]
    public class AllyHealthData : ScriptableObject
    {
        public FloatDataReference BaseMaxHealth;
        public FloatDataReference BaseRegenerationAmount;
        public FloatDataReference BaseRegenerationTime;
        public IntDataReference BaseDefense;
        public FloatDataReference BaseKnockbackResistance;
        public GameObject DamageIndicatorPrefab;
        public FloatDataReference BaseExperienceDropChance;
        public FloatDataReference BaseMagnetDropChance;
        public FloatDataReference BaseExperienceDropAmount;
        public GameObject ExperienceOrbPrefab;
        public GameObject MagnetOrbPrefab;
        public IHealth.TeamType Team;
    } 
}
