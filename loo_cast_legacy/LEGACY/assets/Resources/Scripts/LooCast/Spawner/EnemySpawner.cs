using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Spawner
{
    using Data;
    using Enemy;
    using Random;

    public class EnemySpawner : Spawner
    {
        public EnemySpawnerData Data;

        public float SpawnDelay { get; protected set; }
        public float SpawnTimer { get; protected set; }
        public int MaxEnemies { get; protected set; }
        public GameObject EnemyPrefab { get; protected set; }
        public List<Enemy> SpawnedEnemies { get; protected set; }

        private void Start()
        {
            Initialize(Data);

            SpawnDelay = Data.BaseSpawnDelay.Value;
            SpawnTimer = 0.0f;
            MaxEnemies = Data.BaseMaxEnemies.Value;
            EnemyPrefab = Data.EnemyPrefab;
            SpawnedEnemies = new List<Enemy>();
        }

        protected override void PauseableUpdate()
        {
            SpawnTimer += Time.deltaTime;

            if (SpawnTimer >= SpawnDelay && SpawnedEnemies.Count < MaxEnemies)
            {
                SpawnTimer = 0.0f;
                GameObject spawnedObject = Instantiate(EnemyPrefab, (Vector3)(Random.InsideUnitCircle() * 50.0f) + transform.position, Quaternion.identity, null);
                Enemy spawnedEnemy = spawnedObject.GetComponent<Enemy>();
                SpawnedEnemies.Add(spawnedEnemy);
                spawnedEnemy.OnKilled.AddListener( () => { SpawnedEnemies.Remove(spawnedEnemy); } );
            }
        }
    }
}
