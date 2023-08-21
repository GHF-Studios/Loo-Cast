using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "SpiritStat", menuName = "Data/Attribute/Stat/SpiritStat", order = 0)]
    public class SpiritStat : Stat
    {
        public override string StatName
        {
            get
            {
                return "Spirit";
            }
        }
        
    }
}