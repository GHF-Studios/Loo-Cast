using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "ReflexesStat", menuName = "Data/Attribute/Stat/ReflexesStat", order = 0)]
    public class ReflexesStat : Stat
    {
        public float ConsecutiveProjectileDelayMultiplier
        {
            get
            {
                return 1 + Level.Value * 0.1f;
            }
        }
    } 
}
