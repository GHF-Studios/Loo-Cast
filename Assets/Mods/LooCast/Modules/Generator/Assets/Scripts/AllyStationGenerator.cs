using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Generator
{
    using Data;
    using Random;

    public class AllyStationGenerator : Generator
    {
        public AllyStationGeneratorData Data;

        private int stationCount;

        public override void Initialize()
        {
            stationCount = UnityEngine.Random.Range(Data.MinStationCount.Value, Data.MaxStationCount.Value);

            if (gameObject.activeInHierarchy)
            {
                Generate();
            }
        }

        public override void Generate()
        {
            for (int i = 0; i < stationCount; i++)
            {
                Vector2 spawnPosition = Random.InsideUnitCircle() * Data.SpawnRange.Value;
                GameObject stationObject = Instantiate(Data.AllyStationPrefab, spawnPosition, Quaternion.identity, null);
            }
        }
    } 
}