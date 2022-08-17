using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "FateStat", menuName = "Data/Attribute/Stat/FateStat", order = 0)]
    public class FateStat : Stat
    {
        public override string StatName
        {
            get
            {
                return "Fate";
            }
        }
        public float NegativeEventChanceMultiplier
        {
            get
            {
                return 1 - Level.Value * 0.05f;
            }
        }
    } 
}
