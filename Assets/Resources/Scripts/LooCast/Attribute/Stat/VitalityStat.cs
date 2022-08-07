using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "VitalityStat", menuName = "Data/Attribute/Stat/VitalityStat", order = 0)]
    public class VitalityStat : Stat
    {
        public FloatComputedVariable HealthMultiplier;
    }
}