using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "MightStat", menuName = "Data/Attribute/Stat/MightStat", order = 0)]
    public class MightStat : Stat
    {
        public override string StatName
        {
            get
            {
                return "Might";
            }
        }
        public float DamageMultiplier
        {
            get
            {
                return 1 + Level.Value * 0.1f;
            }
        }
    } 
}
