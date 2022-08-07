using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "ChanceStat", menuName = "Data/Attribute/Stat/ChanceStat", order = 0)]
    public class ChanceStat : Stat
    {
        public FloatComputedVariable RandomChanceMultiplier;
    } 
}
