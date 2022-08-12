using System;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "AgilityStat", menuName = "Data/Attribute/Stat/AgilityStat", order = 0)]
    public class AgilityStat : Stat
    {
        public float MovementSpeedMultiplier
        {
            get
            {
                return 1 + Level.Value * 0.1f;
            }
        }
    } 
}
