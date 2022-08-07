using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "ResistanceStat", menuName = "Data/Attribute/Stat/ResistanceStat", order = 0)]
    public class ResistanceStat : Stat
    {
        public IntComputedVariable DefenseIncrease;
    } 
}
