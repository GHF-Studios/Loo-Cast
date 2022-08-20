using UnityEngine;

namespace LooCast.Asteroid.Data
{
    using LooCast.Data;
    using LooCast.Rarity;

    [CreateAssetMenu(fileName = "AsteroidData", menuName = "Data/Asteroid/AsteroidData", order = 0)]
    public class AsteroidData : ScriptableObject
    {
        public Rarity Rarity;
        public Mesh[] Meshes;
        public AnimationCurve angularSpeedCurve;
        public AnimationCurve speedCurve;
        public AnimationCurve scaleCurve;
    }
}
