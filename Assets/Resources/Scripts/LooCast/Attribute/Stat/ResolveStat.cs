using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "ResolveStat", menuName = "Data/Attribute/Stat/ResolveStat", order = 0)]
    public class ResolveStat : Stat
    {
        public IntComputedVariable PiercingIncrease;
    } 
}
