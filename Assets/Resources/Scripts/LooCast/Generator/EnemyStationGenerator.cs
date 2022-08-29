using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Generator
{
    using Random;
    using Station;
    using Station.Data;

    public class EnemyStationGenerator : Generator
    {
        public int StationCount { get; protected set; }

        [SerializeField] private GameObject enemyStationPrefab;

        public override void Initialize()
        {
            StationCount = 3;
            if (enabled)
            {
                Generate();
            }
        }

        public override void Generate()
        {
            for (int i = 0; i < StationCount; i++)
            {
                Vector2 potentialSpawnPosition = Random.InsideUnitCircle() * 500.0f;
                GameObject stationObject = Instantiate(enemyStationPrefab, potentialSpawnPosition, Quaternion.identity, null);
                EnemyStation station = stationObject.GetComponent<EnemyStation>();
            }
        }
    } 
}
