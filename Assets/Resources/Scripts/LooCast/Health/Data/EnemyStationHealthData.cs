using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Health.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "EnemyStationHealthData", menuName = "Data/Health/EnemyStationHealthData", order = 0)]
    public class EnemyStationHealthData : StationHealthData
    {
        public FloatReference BaseExperienceDropChance;
        public FloatReference BaseExperienceDropAmount;
        public GameObject ExperienceOrbPrefab;
    } 
}
