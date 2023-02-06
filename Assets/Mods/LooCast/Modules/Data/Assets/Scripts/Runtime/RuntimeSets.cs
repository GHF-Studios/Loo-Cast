using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Data.Runtime
{
    using LooCast.Enemy.Data.Runtime;
    using LooCast.Movement.Data.Runtime;
    using LooCast.Station.Data.Runtime;
    using LooCast.Asteroid.Data.Runtime;

    [CreateAssetMenu(fileName = "RuntimeSets", menuName = "Data/Runtime/RuntimeSets", order = 0)]
    public class RuntimeSets : ScriptableObject
    {
        public EnemyRuntimeSet EnemyRuntimeSet;
        public MovementRuntimeSet MovementRuntimeSet;
        public StationRuntimeSet StationRuntimeSet;
        public AsteroidRuntimeSet AsteroidRuntimeSet;

        public void Initialize()
        {
            EnemyRuntimeSet.Initialize();
            MovementRuntimeSet.Initialize();
            StationRuntimeSet.Initialize();
            AsteroidRuntimeSet.Initialize();
        }
    }
}
