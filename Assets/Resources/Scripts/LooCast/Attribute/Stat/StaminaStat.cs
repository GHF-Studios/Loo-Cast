using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "StaminaStat", menuName = "Data/Attribute/Stat/StaminaStat", order = 0)]
    public class StaminaStat : Stat
    {
        public float DurationMultiplier
        {
            get
            {
                return 1 + Level.Value * 0.1f;
            }
        }
    } 
}
