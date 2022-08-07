using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "FateStat", menuName = "Data/Attribute/Stat/FateStat", order = 0)]
    public class FateStat : Stat
    {
        public FloatComputedVariable NegativeEventChanceMultiplier;
    } 
}
