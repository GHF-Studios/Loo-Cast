using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    public class ResolveStat : Stat
    {
        public int PiercingIncrease
        {
            get
            {
                int.TryParse(new DataTable().Compute($"{Level}", "").ToString(), out int value);
                return value;
            }
        }

        public override string ValueToString()
        {
            return $"+{new DataTable().Compute($"{Level}", "")}";
        }
    } 
}
