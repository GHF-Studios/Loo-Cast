using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "PersonalityStat", menuName = "Data/Attribute/Stat/PersonalityStat", order = 0)]
    public class PersonalityStat : Stat
    {
        public FloatComputedVariable ProjectileSpeedMultiplier;
    } 
}
