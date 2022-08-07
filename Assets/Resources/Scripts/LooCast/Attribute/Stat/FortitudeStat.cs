using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Data;

namespace LooCast.Attribute.Stat
{
    using Variable;

    [CreateAssetMenu(fileName = "FortitudeStat", menuName = "Data/Attribute/Stat/FortitudeStat", order = 0)]
    public class FortitudeStat : Stat
    {
        public FloatComputedVariable EnergyConsumptionMultiplier;
    }
}