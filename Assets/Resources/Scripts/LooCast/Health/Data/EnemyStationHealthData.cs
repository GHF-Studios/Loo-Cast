using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Health.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "EnemyStationHealthData", menuName = "Data/Health/EnemyStationHealthData", order = 0)]
    public class EnemyStationHealthData : ScriptableObject
    {
        public FloatDataReference BaseMaxHealth;
        public FloatDataReference BaseRegenerationAmount;
        public FloatDataReference BaseRegenerationTime;
        public IntDataReference BaseDefense;
        public FloatDataReference BaseExperienceDropChance;
        public FloatDataReference BaseExperienceDropAmount;
        public GameObject ExperienceOrbPrefab;
    } 
}
