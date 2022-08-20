using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Generator
{
    using LooCast.Random;
    using LooCast.Asteroid;
    using LooCast.Chance;

    public class AsteroidGenerator : Generator
    {
        [SerializeField] private GameObject asteroidPrefab;
        [SerializeField] private int minAsteroidCount;
        [SerializeField] private int maxAsteroidCount;
        [SerializeField] private int spawnRange;

        private int asteroidCount;

        public override void Initialize()
        {
            asteroidCount = UnityEngine.Random.Range(minAsteroidCount, maxAsteroidCount);

            if (enabled)
            {
                Generate(); 
            }
        }

        public override void Generate()
        {
            for (int i = 0; i < asteroidCount; i++)
            {
                Vector2 spawnPosition = Random.InsideUnitCircle() * spawnRange;
                GameObject asteroidObject = Instantiate(asteroidPrefab, spawnPosition, Quaternion.identity, null);
            }
        }
    } 
}
