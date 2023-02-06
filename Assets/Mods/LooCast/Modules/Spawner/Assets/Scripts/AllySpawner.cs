using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Spawner
{
    using Data;
    using Ally;
    using Random;

    public class AllySpawner : Spawner
    {
        public AllySpawnerData Data;

        public float SpawnDelay { get; protected set; }
        public float SpawnTimer { get; protected set; }
        public int MaxAllies { get; protected set; }
        public GameObject AllyPrefab { get; protected set; }
        public List<Ally> SpawnedAllies { get; protected set; }

        private void Start()
        {
            Initialize(Data);

            SpawnDelay = Data.BaseSpawnDelay.Value;
            SpawnTimer = 0.0f;
            MaxAllies = Data.BaseMaxAllies.Value;
            AllyPrefab = Data.AllyPrefab;
            SpawnedAllies = new List<Ally>();
        }

        protected override void PauseableUpdate()
        {
            SpawnTimer += Time.deltaTime;

            if (SpawnTimer >= SpawnDelay && SpawnedAllies.Count < MaxAllies)
            {
                SpawnTimer = 0.0f;
                GameObject spawnedObject = Instantiate(AllyPrefab, (Vector3)(Random.InsideUnitCircle() * 50.0f) + transform.position, Quaternion.identity, null);
                Ally spawnedAlly = spawnedObject.GetComponent<Ally>();
                SpawnedAllies.Add(spawnedAlly);
                spawnedAlly.OnKilled.AddListener( () => { SpawnedAllies.Remove(spawnedAlly); } );
            }
        }
    }
}
