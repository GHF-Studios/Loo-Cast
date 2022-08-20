using UnityEngine;

namespace LooCast.Asteroid.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "AsteroidData", menuName = "Data/Asteroid/AsteroidData", order = 0)]
    public class AsteroidData : ScriptableObject
    {
        public Mesh[] Meshes;
        public Material[] Materials;
        public int[] RarityWeights;
        public int[] SizeWeights;
        public AnimationCurve AngularSpeedDistribution;
        public AnimationCurve SpeedDistribution;
        public AnimationCurve ScaleDistribution;
    }
}
