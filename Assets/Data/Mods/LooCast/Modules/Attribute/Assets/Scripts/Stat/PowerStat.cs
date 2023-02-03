using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "PowerStat", menuName = "Data/Attribute/Stat/PowerStat", order = 0)]
    public class PowerStat : Stat
    {
        public override string StatName
        {
            get
            {
                return "Power";
            }
        }
        public float KnockbackMultiplier
        {
            get
            {
                return 1 + Level.Value * 0.1f;
            }
        }
    }
}