using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Station.Data
{
    using LooCast.Health.Data;

    [CreateAssetMenu(fileName = "EnemyStationData", menuName = "Data/Station/EnemyStationData", order = 0)]
    public sealed class EnemyStationData : StationData
    {
        public EnemyStationHealthData HealthData;
    } 
}
