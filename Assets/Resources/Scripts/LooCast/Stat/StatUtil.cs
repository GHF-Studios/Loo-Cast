using System;
using System.Collections.Generic;

namespace LooCast.Stat
{
    public static class StatUtil
    {
        /// <summary>
        /// Applies Permanent Increases, then Permanent Multipliers, then Temporary Increases and then Temporary Multipliers
        /// </summary>
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

        /// <summary>
        /// Applies Permanent Increases, then Permanent Multipliers, then Temporary Increases and then Temporary Multipliers
        /// </summary>
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

        /// <summary>
        /// Ignores all Multipliers and Increases
        /// </summary>
        public static Func<List<Multiplier>, List<Increase>, List<TemporaryMultiplier>, List<TemporaryIncrease>, float, float> StaticFloatStatEvaluator = (permanentMultipliers, permanentIncreases, multipliers, increases, baseValue) =>
        {
            return baseValue;
        };

        /// <summary>
        /// Ignores all Multipliers and Increases
        /// </summary>
        public static Func<List<Multiplier>, List<Increase>, List<TemporaryMultiplier>, List<TemporaryIncrease>, int, int> StaticIntStatEvaluator = (permanentMultipliers, permanentIncreases, multipliers, increases, baseValue) =>
        {
            return baseValue;
        };
    }
}
