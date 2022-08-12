using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "ResistanceStat", menuName = "Data/Attribute/Stat/ResistanceStat", order = 0)]
    public class ResistanceStat : Stat
    {
        public int DefenseIncrease
        {
            get
            {
                return Level.Value;
            }
        }
    } 
}
