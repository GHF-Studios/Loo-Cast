using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "IntellectStat", menuName = "Data/Attribute/Stat/IntellectStat", order = 0)]
    public class IntellectStat : Stat
    {
        public FloatComputedVariable ExperienceMultiplier;
    } 
}
