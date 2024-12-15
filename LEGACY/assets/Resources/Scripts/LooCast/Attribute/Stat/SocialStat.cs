using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "SocialStat", menuName = "Data/Attribute/Stat/SocialStat", order = 0)]
    public class SocialStat : Stat
    {
        public override string StatName
        {
            get
            {
                return "Social";
            }
        }
        
    }
}
