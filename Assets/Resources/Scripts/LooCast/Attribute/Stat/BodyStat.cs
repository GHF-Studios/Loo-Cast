using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "BodyStat", menuName = "Data/Attribute/Stat/BodyStat", order = 0)]
    public class BodyStat : Stat
    {
        public float EnergyMultiplier
        {
            get
            {
                return 1 + Level.Value * 0.1f;
            }
        }
    } 
}
