using System;
using System.Collections.Generic;
using UnityEngine.Events;

namespace LooCast.Variable
{
    public class FloatComputedVariable : ComputedVariable<float>
    {
        public FloatComputedVariable(float baseValue) : base(baseValue, ComputedVariableUtil.DefaultFloatStatEvaluator)
        {

        }

        public FloatComputedVariable(float baseValue, Func<List<Multiplier>, List<Increase>, List<TemporaryMultiplier>, List<TemporaryIncrease>, float, float> valueEvaluator) : base(baseValue, valueEvaluator)
        {

        }
    }
}
