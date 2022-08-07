using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "RecoveryStat", menuName = "Data/Attribute/Stat/RecoveryStat", order = 0)]
    public class RecoveryStat : Stat
    {
        public FloatComputedVariable HealthRegenrationMultiplier;
    } 
}
