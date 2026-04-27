using UnityEngine;

namespace LooCast.Generator.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "AllyStationGeneratorData", menuName = "Data/Generator/AllyStationGeneratorData", order = 0)]
    public class AllyStationGeneratorData : ScriptableObject
    {
        public GameObject AllyStationPrefab;
        public IntDataReference MinStationCount;
        public IntDataReference MaxStationCount;
        public IntDataReference SpawnRange;
    }
}
