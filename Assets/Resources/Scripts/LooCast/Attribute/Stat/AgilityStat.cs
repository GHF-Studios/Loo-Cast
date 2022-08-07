using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    using Variable;
    using Util;

    [CreateAssetMenu(fileName = "AgilityStat", menuName = "Data/Attribute/Stat/AgilityStat", order = 0)]
    public class AgilityStat : Stat
    {
        public FloatComputedVariable MovementSpeedMultiplier;
    } 
}
