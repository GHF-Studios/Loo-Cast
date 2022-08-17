using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "SenseStat", menuName = "Data/Attribute/Stat/SenseStat", order = 0)]
    public class SenseStat : Stat
    {
        public override string StatName
        {
            get
            {
                return "Sense";
            }
        }
        
    }
}
