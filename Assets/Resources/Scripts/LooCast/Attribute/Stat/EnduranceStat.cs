using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "EnduranceStat", menuName = "Data/Attribute/Stat/EnduranceStat", order = 0)]
    public class EnduranceStat : Stat
    {
        public float EnergyRegenerationMultiplier
        {
            get
            {
                return 1 + Level.Value * 0.05f;
            }
        }
    } 
}
