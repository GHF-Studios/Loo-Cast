using UnityEngine;

namespace LooCast.Generator.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "AsteroidGeneratorData", menuName = "Data/Generator/AsteroidGeneratorData", order = 0)]
    public class AsteroidGeneratorData : ScriptableObject
    {
        public GameObject AsteroidPrefab;
        public IntDataReference MinAsteroidCount;
        public IntDataReference MaxAsteroidCount;
        public IntDataReference SpawnRange;
    }
}
