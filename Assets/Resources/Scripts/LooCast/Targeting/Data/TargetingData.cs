using UnityEngine;

namespace LooCast.Targeting.Data
{
    using LooCast.Data;

    public class TargetingData : ScriptableObject
    {
        public FloatReference BaseRadius;
        public string[] TargetedTags;
        public BoolReference DrawGizmos;
    }
}
