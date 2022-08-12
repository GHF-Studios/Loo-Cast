using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "SanityStat", menuName = "Data/Attribute/Stat/SanityStat", order = 0)]
    public class SanityStat : Stat
    {
        public float ProjectileSizeMultiplier
        {
            get
            {
                return 1 + Level.Value * 0.1f;
            }
        }
    } 
}
