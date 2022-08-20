using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Generator
{
    using LooCast.Random;
    using LooCast.Asteroid;

    public class AsteroidGenerator : Generator
    {
        [SerializeField] private GameObject[] prefabs;
        [SerializeField] private int minAsteroidCount;
        [SerializeField] private int maxAsteroidCount;
        [SerializeField] private int spawnRange;

        private int asteroidCount;

        public override void Initialize()
        {
            asteroidCount = UnityEngine.Random.Range(minAsteroidCount, maxAsteroidCount);
            Generate();
        }

        public override void Generate()
        {
            for (int i = 0; i < asteroidCount; i++)
            {
                Vector2 spawnPosition = Random.InsideUnitCircle() * spawnRange;
                int randomPrefabIndex = UnityEngine.Random.Range(0, prefabs.Length - 1);
                GameObject asteroidObject = Instantiate(prefabs[randomPrefabIndex], spawnPosition, Quaternion.identity, null);
            }
        }
    } 
}
