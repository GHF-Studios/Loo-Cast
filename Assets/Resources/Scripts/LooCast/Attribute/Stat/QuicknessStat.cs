using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "QuicknessStat", menuName = "Data/Attribute/Stat/QuicknessStat", order = 0)]
    public class QuicknessStat : Stat
    {
        public FloatComputedVariable AttackDelayMultiplier;
    } 
}
