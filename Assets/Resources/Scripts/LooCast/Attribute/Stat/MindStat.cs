using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "MindStat", menuName = "Data/Attribute/Stat/MindStat", order = 0)]
    public class MindStat : Stat
    {
        public FloatComputedVariable RangeMultiplier;
    } 
}
