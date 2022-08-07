using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "FortuneStat", menuName = "Data/Attribute/Stat/FortuneStat", order = 0)]
    public class FortuneStat : Stat
    {
        public FloatComputedVariable PositiveEventChanceMultiplier;
    } 
}
