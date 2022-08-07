using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "SanityStat", menuName = "Data/Attribute/Stat/SanityStat", order = 0)]
    public class SanityStat : Stat
    {
        public FloatComputedVariable ProjectileSizeMultiplier;
    } 
}
