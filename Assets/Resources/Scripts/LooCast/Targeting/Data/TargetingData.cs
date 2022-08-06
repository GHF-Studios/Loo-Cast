using UnityEngine;

namespace LooCast.Targeting.Data
{
    using LooCast.Data;

    public class TargetingData : ScriptableObject
    {
        public FloatDataReference BaseRadius;
        public string[] TargetedTags;
        public BoolDataReference DrawGizmos;
    }
}
