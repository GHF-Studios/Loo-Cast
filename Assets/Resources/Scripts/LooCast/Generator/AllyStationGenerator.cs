using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Generator
{
    using Random;

    public class AllyStationGenerator : Generator
    {
        [SerializeField] private GameObject allyStationPrefab;
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
                GameObject stationObject = Instantiate(allyStationPrefab, spawnPosition, Quaternion.identity, null);
            }
        }
    } 
}
