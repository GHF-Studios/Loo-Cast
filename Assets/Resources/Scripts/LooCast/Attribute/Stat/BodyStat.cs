using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "BodyStat", menuName = "Data/Attribute/Stat/BodyStat", order = 0)]
    public class BodyStat : Stat
    {
        public FloatComputedVariable EnergyMultiplier;
    } 
}
