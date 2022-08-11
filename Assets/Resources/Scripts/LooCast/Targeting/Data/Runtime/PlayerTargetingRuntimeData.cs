using UnityEngine;
using System.Collections.Generic;

namespace LooCast.Targeting.Data.Runtime
{
    using LooCast.Variable;
    using LooCast.Target;

    [CreateAssetMenu(fileName = "PlayerTargetingRuntimeData", menuName = "Data/Targeting/Runtime/PlayerTargetingRuntimeData", order = 0)]
    public class PlayerTargetingRuntimeData : ScriptableObject
    {
        public FloatComputedVariable Radius;
        public StringVariable[] TargetTags;
        public BoolVariable DrawGizmos;
        public System.Random Random;
        public List<Target> IgnoredTargets;
    }
}
