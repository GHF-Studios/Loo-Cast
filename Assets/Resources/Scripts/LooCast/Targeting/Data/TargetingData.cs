using UnityEngine;

namespace LooCast.Targeting.Data
{
    using LooCast.Data;

    public abstract class TargetingData : ScriptableObject
    {
        public FloatDataReference Radius;
        public StringDataReference[] TargetedTags;
        public BoolDataReference DrawGizmos;
    }
}
