using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "ResolveStat", menuName = "Data/Attribute/Stat/ResolveStat", order = 0)]
    public class ResolveStat : Stat
    {
        public override string StatName
        {
            get
            {
                return "Resolve";
            }
        }
        public int PiercingIncrease
        {
            get
            {
                return Level.Value;
            }
        }
    } 
}
