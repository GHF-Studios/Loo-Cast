using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "ResilienceStat", menuName = "Data/Attribute/Stat/ResilienceStat", order = 0)]
    public class ResilienceStat : Stat
    {
        public IntComputedVariable ShieldStrengthIncrease;
    } 
}
