using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "StaminaStat", menuName = "Data/Attribute/Stat/StaminaStat", order = 0)]
    public class StaminaStat : Stat
    {
        public FloatComputedVariable DurationMultiplier;
    } 
}
