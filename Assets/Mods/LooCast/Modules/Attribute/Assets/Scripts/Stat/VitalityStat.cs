using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "VitalityStat", menuName = "Data/Attribute/Stat/VitalityStat", order = 0)]
    public class VitalityStat : Stat
    {
        public override string StatName
        {
            get
            {
                return "Vitality";
            }
        }
        public float HealthMultiplier
        {
            get
            {
                return 1 + Level.Value * 0.1f;
            }
        }
    }
}