using System;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "AgilityStat", menuName = "Data/Attribute/Stat/AgilityStat", order = 0)]
    [Serializable]
    public class AgilityStat : Stat
    {
        public FloatComputedVariable MovementSpeedMultiplier;
    } 
}
