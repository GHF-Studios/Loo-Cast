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
        [SerializeField] private GameObject enemyStationPrefab;
        [SerializeField] private int minStationCount;
        [SerializeField] private int maxStationCount;
        [SerializeField] private int spawnRange;

        private int stationCount;

        public override void Initialize()
        {
            stationCount = UnityEngine.Random.Range(minStationCount, maxStationCount);

            if (gameObject.activeInHierarchy)
            {
                Generate();
            }
        }

        public override void Generate()
        {
            for (int i = 0; i < stationCount; i++)
            {
                Vector2 spawnPosition = Random.InsideUnitCircle() * spawnRange;
                GameObject stationObject = Instantiate(enemyStationPrefab, spawnPosition, Quaternion.identity, null);
            }
        }
    } 
}
