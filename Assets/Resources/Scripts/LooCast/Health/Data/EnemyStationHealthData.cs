using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Health.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "EnemyStationHealthData", menuName = "Data/Health/EnemyStationHealthData", order = 0)]
    public class EnemyStationHealthData : StationHealthData
    {
        public FloatDataReference BaseExperienceDropChance;
        public FloatDataReference BaseExperienceDropAmount;
        public GameObject ExperienceOrbPrefab;
    } 
}
