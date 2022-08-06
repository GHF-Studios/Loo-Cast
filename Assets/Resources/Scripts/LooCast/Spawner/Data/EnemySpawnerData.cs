using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Spawner.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "EnemySpawnerData", menuName = "Data/Spawner/EnemySpawnerData", order = 0)]
    public sealed class EnemySpawnerData : SpawnerData
    {
        public FloatDataReference BaseSpawnDelay;
        public IntDataReference BaseMaxEnemies;
        public GameObject EnemyPrefab;
    } 
}
