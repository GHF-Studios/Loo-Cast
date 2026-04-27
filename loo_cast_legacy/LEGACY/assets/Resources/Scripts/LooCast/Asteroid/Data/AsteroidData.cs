using UnityEngine;

namespace LooCast.Asteroid.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "AsteroidData", menuName = "Data/Asteroid/AsteroidData", order = 0)]
    public class AsteroidData : ScriptableObject
    {
        public int[] SizeWeights;
        public int[] RarityWeights;
        public AsteroidSizeData[] AsteroidSizeDatas;
        public AsteroidRarityData[] AsteroidRarityDatas;
        public AnimationCurve AngularSpeed;
        public AnimationCurve Speed;
        public AnimationCurve Scale;
    }
}
