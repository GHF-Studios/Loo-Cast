using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "CautiousnessStat", menuName = "Data/Attribute/Stat/CautiousnessStat", order = 0)]
    public class CautiousnessStat : Stat
    {
        public override string StatName
        {
            get
            {
                return "Cautiousness";
            }
        }
        
    }
}