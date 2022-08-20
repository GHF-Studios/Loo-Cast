using UnityEngine;

namespace LooCast.Asteroid.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "AsteroidData", menuName = "Data/Asteroid/AsteroidData", order = 0)]
    public class AsteroidData : ScriptableObject
    {
        public AnimationCurve angularSpeedCurve;
        public AnimationCurve speedCurve;
        public AnimationCurve scaleCurve;
    }
}
