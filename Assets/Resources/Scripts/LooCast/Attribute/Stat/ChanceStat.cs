using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "ChanceStat", menuName = "Data/Attribute/Stat/ChanceStat", order = 0)]
    public class ChanceStat : Stat
    {
        public float RandomChanceMultiplier
        {
            get
            {
                return 1 + Level.Value * 0.1f;
            }
        }
    } 
}
