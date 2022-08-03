using System;
using System.Collections.Generic;

namespace LooCast.Stat
{
    public static class StatUtil
    {
        public static Func<List<Multiplier>, List<Increase>, List<TemporaryMultiplier>, List<TemporaryIncrease>, float, float> DefaultFloatStatEvaluator = (permanentMultipliers, permanentIncreases, multipliers, increases, baseValue) =>
        {
            float value = baseValue;

            foreach (Increase permanentIncrease in permanentIncreases)
            {
                value += permanentIncrease.Value;
            }

            foreach (Multiplier permanentMultiplier in permanentMultipliers)
            {
                value *= permanentMultiplier.Value;
            }

            foreach (Increase increase in increases)
            {
                value += increase.Value;
            }

            foreach (Multiplier multiplier in multipliers)
            {
                value *= multiplier.Value;
            }

            return value;
        };

        public static Func<List<Multiplier>, List<Increase>, List<TemporaryMultiplier>, List<TemporaryIncrease>, int, int> DefaultIntStatEvaluator = (permanentMultipliers, permanentIncreases, multipliers, increases, baseValue) =>
        {
            float value = baseValue;

            foreach (Increase permanentIncrease in permanentIncreases)
            {
                value += permanentIncrease.Value;
            }

            foreach (Multiplier permanentMultiplier in permanentMultipliers)
            {
                value *= permanentMultiplier.Value;
            }

            foreach (Increase increase in increases)
            {
                value += increase.Value;
            }

            foreach (Multiplier multiplier in multipliers)
            {
                value *= multiplier.Value;
            }

            return (int)value;
        };
    }
}
