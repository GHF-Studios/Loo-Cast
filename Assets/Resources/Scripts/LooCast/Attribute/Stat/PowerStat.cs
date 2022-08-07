using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "PowerStat", menuName = "Data/Attribute/Stat/PowerStat", order = 0)]
    public class PowerStat : Stat
    {
        public FloatComputedVariable KnockbackMultiplier;
    }
}