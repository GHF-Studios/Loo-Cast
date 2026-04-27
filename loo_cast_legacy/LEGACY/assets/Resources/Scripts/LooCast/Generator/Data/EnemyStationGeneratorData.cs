using UnityEngine;

namespace LooCast.Generator.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "EnemyStationGeneratorData", menuName = "Data/Generator/EnemyStationGeneratorData", order = 0)]
    public class EnemyStationGeneratorData : ScriptableObject
    {
        public GameObject EnemyStationPrefab;
        public IntDataReference MinStationCount;
        public IntDataReference MaxStationCount;
        public IntDataReference SpawnRange;
    }
}
