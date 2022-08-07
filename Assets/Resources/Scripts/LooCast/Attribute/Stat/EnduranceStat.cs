using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "EnduranceStat", menuName = "Data/Attribute/Stat/EnduranceStat", order = 0)]
    public class EnduranceStat : Stat
    {
        public FloatComputedVariable EnergyRegenerationMultiplier;
    } 
}
