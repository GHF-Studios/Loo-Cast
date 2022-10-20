using System;
using System.Collections.Generic;

namespace LooCast.Variable
{
    public static class ComputedVariableUtil
    {
        /// <summary>
        /// Applies Increases, then Multipliers
        /// </summary>
        public static Func<List<Multiplier>, List<Increase>, List<TemporaryMultiplier>, List<TemporaryIncrease>, float, float> DefaultFloatStatEvaluator = (permanentMultipliers, permanentIncreases, multipliers, increases, baseValue) =>
        {
            float value = baseValue;

            foreach (Increase permanentIncrease in permanentIncreases)
            {
                value += permanentIncrease.Value;
            }

            foreach (Increase increase in increases)
            {
                value += increase.Value;
            }

            foreach (Multiplier permanentMultiplier in permanentMultipliers)
            {
                value *= permanentMultiplier.Value;
            }

            foreach (Multiplier multiplier in multipliers)
            {
                value *= multiplier.Value;
            }

            return value;
        };

        /// <summary>
        /// Applies Increases, then Multipliers
        /// </summary>
        public static Func<List<Multiplier>, List<Increase>, List<TemporaryMultiplier>, List<TemporaryIncrease>, int, int> DefaultIntStatEvaluator = (permanentMultipliers, permanentIncreases, multipliers, increases, baseValue) =>
        {
            float value = baseValue;

            foreach (Increase permanentIncrease in permanentIncreases)
            {
                value += permanentIncrease.Value;
            }

            foreach (Increase increase in increases)
            {
                value += increase.Value;
            }

            foreach (Multiplier permanentMultiplier in permanentMultipliers)
            {
                value *= permanentMultiplier.Value;
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
