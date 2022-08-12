using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "BrawnStat", menuName = "Data/Attribute/Stat/BrawnStat", order = 0)]
    public class BrawnStat : Stat
    {
        public int ArmorPenetrationIncrease
        {
            get
            {
                return Level.Value * 5;
            }
        }
    } 
}
