using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Data.Runtime
{
    using LooCast.Core.Data.Runtime;
    using LooCast.Enemy.Data.Runtime;
    using LooCast.Movement.Data.Runtime;
    using LooCast.Station.Data.Runtime;

    [CreateAssetMenu(fileName = "RuntimeSets", menuName = "Data/Runtime/RuntimeSets", order = 0)]
    public class RuntimeSets : ScriptableObject
    {
        public ExtendedMonoBehaviourRuntimeSet ExtendedMonoBehaviourRuntimeSet;
        public EnemyRuntimeSet EnemyRuntimeSet;
        public MovementRuntimeSet MovementRuntimeSet;
        public StationRuntimeSet StationRuntimeSet;

        public void Initialize()
        {
            ExtendedMonoBehaviourRuntimeSet.Initialize();
            EnemyRuntimeSet.Initialize();
            MovementRuntimeSet.Initialize();
            StationRuntimeSet.Initialize();
        }
    }
}
