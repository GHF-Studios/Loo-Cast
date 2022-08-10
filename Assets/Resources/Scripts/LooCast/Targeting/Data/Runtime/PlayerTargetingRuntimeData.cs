using UnityEngine;

namespace LooCast.Targeting.Data.Runtime
{
    using LooCast.Variable;

    [CreateAssetMenu(fileName = "PlayerTargetingRuntimeData", menuName = "Data/Targeting/PlayerTargetingRuntimeData", order = 0)]
    public class PlayerTargetingRuntimeData : ScriptableObject
    {
        public FloatComputedVariable Radius;
        public StringVariable[] TargetTags;
        public BoolVariable DrawGizmos;
    }
}
