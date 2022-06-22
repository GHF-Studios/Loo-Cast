using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    public class SanityStat : Stat
    {
        public float ProjectileSizeMultiplier
        {
            get
            {
                float.TryParse(new DataTable().Compute($"1 + ({Level} * 0.1)", "").ToString(), out float value);
                return value;
            }
        }

        public override string ValueToString()
        {
            return $"+{new DataTable().Compute($"{Level} * 10", "")}%";
        }
    } 
}
