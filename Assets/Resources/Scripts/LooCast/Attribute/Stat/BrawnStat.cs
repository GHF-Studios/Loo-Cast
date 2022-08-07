using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "BrawnStat", menuName = "Data/Attribute/Stat/BrawnStat", order = 0)]
    public class BrawnStat : Stat
    {
        public IntComputedVariable ArmorPenetrationIncrease;
    } 
}
