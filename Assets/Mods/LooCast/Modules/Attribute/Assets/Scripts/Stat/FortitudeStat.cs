using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    [CreateAssetMenu(fileName = "FortitudeStat", menuName = "Data/Attribute/Stat/FortitudeStat", order = 0)]
    public class FortitudeStat : Stat
    {
        public override string StatName
        {
            get
            {
                return "Fortitude";
            }
        }
        public float EnergyConsumptionMultiplier
        {
            get
            {
                return 1 + Level.Value * 0.05f;
            }
        }
    }
}