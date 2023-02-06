using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Generator
{
    using Data;
    using Random;
    using Asteroid;
    using Chance;

    public class AsteroidGenerator : Generator
    {
        public AsteroidGeneratorData Data;

        private int asteroidCount;

        public override void Initialize()
        {
            asteroidCount = UnityEngine.Random.Range(Data.MinAsteroidCount.Value, Data.MaxAsteroidCount.Value);

            if (gameObject.activeInHierarchy)
            {
                Generate();
            }
        }

        public override void Generate()
        {
            for (int i = 0; i < asteroidCount; i++)
            {
                Vector2 spawnPosition = Random.InsideUnitCircle() * Data.SpawnRange.Value;
                GameObject asteroidObject = Instantiate(Data.AsteroidPrefab, spawnPosition, Quaternion.identity, null);
            }
        }
    } 
}
