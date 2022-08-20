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
        [SerializeField] private GameObject[] asteroidPrefabs;
        [SerializeField] private int[] asteroidSpawnWeights;

        [SerializeField] private int minAsteroidCount;
        [SerializeField] private int maxAsteroidCount;
        [SerializeField] private int spawnRange;

        private int asteroidCount;

        public override void Initialize()
        {
            asteroidCount = UnityEngine.Random.Range(minAsteroidCount, maxAsteroidCount);

            if (asteroidPrefabs.Length != asteroidSpawnWeights.Length)
            {
                throw new System.Exception("Must have same amount of prefabs and spawn weights!");
            }

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
                int weightedRandomPrefabIndex = Chance.GetRandomWeightedIndex(asteroidSpawnWeights);
                GameObject asteroidObject = Instantiate(asteroidPrefabs[weightedRandomPrefabIndex], spawnPosition, Quaternion.identity, null);
            }
        }

        
    } 
}
