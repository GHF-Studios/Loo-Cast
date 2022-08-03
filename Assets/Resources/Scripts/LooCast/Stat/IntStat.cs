using System;
using System.Collections.Generic;
using UnityEngine.Events;

namespace LooCast.Stat
{
    public class IntStat : Stat<int>
    {
        public IntStat(int baseValue) : base(baseValue, StatUtil.DefaultIntStatEvaluator)
        {

        }

        public IntStat(int baseValue, Func<List<Multiplier>, List<Increase>, List<TemporaryMultiplier>, List<TemporaryIncrease>, int, int> valueEvaluator) : base(baseValue, valueEvaluator)
        {

        }
    }
}
