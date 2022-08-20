using UnityEngine;

namespace LooCast.Asteroid.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "AsteroidSizeData", menuName = "Data/Asteroid/AsteroidSizeData", order = 0)]
    public class AsteroidSizeData : ScriptableObject
    {
        public Mesh[] Meshes;
    }
}
