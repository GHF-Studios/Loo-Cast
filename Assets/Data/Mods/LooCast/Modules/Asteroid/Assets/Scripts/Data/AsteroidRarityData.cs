using UnityEngine;

namespace LooCast.Asteroid.Data
{
    using LooCast.Data;
    using LooCast.Resource;

    [CreateAssetMenu(fileName = "AsteroidRarityData", menuName = "Data/Asteroid/AsteroidRarityData", order = 0)]
    public class AsteroidRarityData : ScriptableObject
    {
        public Resource[] Resources;
        public AnimationCurve[] DepositWeights;
        public Material Material;
    }
}
