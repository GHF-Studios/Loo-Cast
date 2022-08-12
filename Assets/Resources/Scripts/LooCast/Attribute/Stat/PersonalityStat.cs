using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "PersonalityStat", menuName = "Data/Attribute/Stat/PersonalityStat", order = 0)]
    public class PersonalityStat : Stat
    {
        public float ProjectileSpeedMultiplier
        {
            get
            {
                return 1 + Level.Value * 0.1f;
            }
        }
    } 
}
