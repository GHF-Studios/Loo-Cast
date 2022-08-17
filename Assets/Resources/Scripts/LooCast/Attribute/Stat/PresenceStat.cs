using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "PresenceStat", menuName = "Data/Attribute/Stat/PresenceStat", order = 0)]
    public class PresenceStat : Stat
    {
        public override string StatName
        {
            get
            {
                return "Presence";
            }
        }
        
    }
}
