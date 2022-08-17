using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "EgoStat", menuName = "Data/Attribute/Stat/EgoStat", order = 0)]
    public class EgoStat : Stat
    {
        public override string StatName
        {
            get
            {
                return "Ego";
            }
        }
        public float DamageReflection
        {
            get
            {
                return 1 + Level.Value * 0.1f;
            }
        }
    } 
}
