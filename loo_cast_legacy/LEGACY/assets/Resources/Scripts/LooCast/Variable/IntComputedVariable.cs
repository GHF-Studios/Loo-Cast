using System;
using System.Collections.Generic;
using UnityEngine.Events;

namespace LooCast.Variable
{
    [Serializable]
    public class IntComputedVariable : ComputedVariable<int>
    {
        public IntComputedVariable(int baseValue) : base(baseValue, ComputedVariableUtil.DefaultIntStatEvaluator)
        {

        }

        public IntComputedVariable(int baseValue, Func<List<Multiplier>, List<Increase>, List<TemporaryMultiplier>, List<TemporaryIncrease>, int, int> valueEvaluator) : base(baseValue, valueEvaluator)
        {

        }
    }
}
