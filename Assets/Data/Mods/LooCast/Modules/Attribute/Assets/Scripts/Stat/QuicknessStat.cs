using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "QuicknessStat", menuName = "Data/Attribute/Stat/QuicknessStat", order = 0)]
    public class QuicknessStat : Stat
    {
        public override string StatName
        {
            get
            {
                return "Quickness";
            }
        }
        public float AttackDelayMultiplier
        {
            get
            {
                return 1 - Level.Value * 0.075f;
            }
        }
    } 
}
