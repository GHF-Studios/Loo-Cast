using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "MindStat", menuName = "Data/Attribute/Stat/MindStat", order = 0)]
    public class MindStat : Stat
    {
        public float RangeMultiplier
        {
            get
            {
                return 1 + Level.Value * 0.1f;
            }
        }
    } 
}
