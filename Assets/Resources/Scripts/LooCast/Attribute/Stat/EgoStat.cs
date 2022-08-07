using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "EgoStat", menuName = "Data/Attribute/Stat/EgoStat", order = 0)]
    public class EgoStat : Stat
    {
        public FloatComputedVariable DamageReflection;
    } 
}
