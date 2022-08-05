using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Data.Runtime
{
    using Enemy.Data.Runtime;

    [CreateAssetMenu(fileName = "RuntimeSets", menuName = "Data/Runtime/RuntimeSets", order = 0)]
    public class RuntimeSets : ScriptableObject
    {
        public EnemyRuntimeSet EnemyRuntimeSet;

        public void Initialize()
        {
            EnemyRuntimeSet.Initialize();
        }
    }
}
