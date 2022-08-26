using UnityEngine;

namespace LooCast.Targeting.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "TargetingData", menuName = "Data/Targeting/TargetingData", order = 0)]
    public class TargetingData : ScriptableObject
    {
        public FloatDataReference Radius;
        public StringDataReference[] TargetedTags;
        public BoolDataReference DrawGizmos;
    }
}
