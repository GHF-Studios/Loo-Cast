using System;
using System.Collections.Generic;
using UnityEngine.Events;

namespace LooCast.Stat
{
    public class FloatStat : Stat<float>
    {
        public FloatStat(float baseValue) : base(baseValue, StatUtil.DefaultFloatStatEvaluator)
        {

        }

        public FloatStat(float baseValue, Func<List<Multiplier>, List<Increase>, List<TemporaryMultiplier>, List<TemporaryIncrease>, float, float> valueEvaluator) : base(baseValue, valueEvaluator)
        {

        }
    }
}
