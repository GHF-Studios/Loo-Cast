using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Spawner.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "AllySpawnerData", menuName = "Data/Spawner/AllySpawnerData", order = 0)]
    public sealed class AllySpawnerData : SpawnerData
    {
        public FloatDataReference BaseSpawnDelay;
        public IntDataReference BaseMaxAllies;
        public GameObject AllyPrefab;
    } 
}
