using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "MightStat", menuName = "Data/Attribute/Stat/MightStat", order = 0)]
    public class MightStat : Stat
    {
        public FloatComputedVariable DamageMultiplier;
    } 
}
