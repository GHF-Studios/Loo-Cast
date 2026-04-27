using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "FortuneStat", menuName = "Data/Attribute/Stat/FortuneStat", order = 0)]
    public class FortuneStat : Stat
    {
        public override string StatName
        {
            get
            {
                return "Fortune";
            }
        }
        public float PositiveEventChanceMultiplier
        {
            get
            {
                return 1 + Level.Value * 0.1f;
            }
        }
    } 
}
