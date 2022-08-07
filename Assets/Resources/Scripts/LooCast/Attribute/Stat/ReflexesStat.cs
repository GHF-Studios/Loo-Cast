using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "ReflexesStat", menuName = "Data/Attribute/Stat/ReflexesStat", order = 0)]
    public class ReflexesStat : Stat
    {
        public FloatComputedVariable ConsecutiveProjectileDelayMultiplier;
    } 
}
