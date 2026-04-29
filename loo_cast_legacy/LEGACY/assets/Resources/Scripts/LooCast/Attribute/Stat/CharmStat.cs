using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "CharmStat", menuName = "Data/Attribute/Stat/CharmStat", order = 0)]
    public class CharmStat : Stat
    {
        public override string StatName
        {
            get
            {
                return "Charm";
            }
        }
        
    }
}
