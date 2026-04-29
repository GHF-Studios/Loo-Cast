using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "ResilienceStat", menuName = "Data/Attribute/Stat/ResilienceStat", order = 0)]
    public class ResilienceStat : Stat
    {
        public override string StatName
        {
            get
            {
                return "Resilience";
            }
        }

        public float KnockbackResistanceMultiplier
        {
            get
            {
                return 1.0f + (Level.Value * 0.1f);
            }
        }
    } 
}
