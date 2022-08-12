using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "IntellectStat", menuName = "Data/Attribute/Stat/IntellectStat", order = 0)]
    public class IntellectStat : Stat
    {
        public float ExperienceMultiplier
        {
            get
            {
                return 1 + Level.Value * 0.1f;
            }
        }
    } 
}
