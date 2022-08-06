using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Health.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "EnemyHealthData", menuName = "Data/Health/EnemyHealthData", order = 0)]
    public class EnemyHealthData : HealthData
    {
        public FloatDataReference BaseExperienceDropChance;
        public FloatDataReference BaseMagnetDropChance;
        public FloatDataReference BaseExperienceDropAmount;
        public GameObject ExperienceOrbPrefab;
        public GameObject MagnetOrbPrefab;
    } 
}
