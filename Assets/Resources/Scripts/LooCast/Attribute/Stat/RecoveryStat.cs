using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "RecoveryStat", menuName = "Data/Attribute/Stat/RecoveryStat", order = 0)]
    public class RecoveryStat : Stat
    {
        public float HealthRegenrationMultiplier
        {
            get
            {
                return 1 + Level.Value * 0.1f;
            }
        }
    } 
}
