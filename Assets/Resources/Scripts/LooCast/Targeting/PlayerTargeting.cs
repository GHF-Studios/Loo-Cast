using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace LooCast.Targeting
{
    using Data;
    using Data.Runtime;
    using LooCast.Variable;

    public class PlayerTargeting : Targeting
    {
        public PlayerTargetingData Data;
        public PlayerTargetingRuntimeData RuntimeData;

        private void Start()
        {
            Initialize(Data);

            RuntimeData.Radius = new FloatComputedVariable(Data.Radius.Value);
            RuntimeData.TargetTags = new StringVariable[Data.TargetedTags.Length];
            for (int i = 0; i < RuntimeData.TargetTags.Length; i++)
            {
                RuntimeData.TargetTags[i] = new StringVariable(Data.TargetedTags[i].Value);
            }
            RuntimeData.DrawGizmos = new BoolVariable(Data.DrawGizmos.Value);
        }
    }
}
