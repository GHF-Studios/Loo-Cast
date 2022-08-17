using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "WitsStat", menuName = "Data/Attribute/Stat/WitsStat", order = 0)]
    public class WitsStat : Stat
    {
        public override string StatName
        {
            get
            {
                return "Wits";
            }
        }
        
    }
}
