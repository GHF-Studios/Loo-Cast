using CSSystem = System;
using UnityEngine;

namespace LooCast.Random
{
    public class SeededRandom
    {
        private CSSystem.Random prng;

        public SeededRandom(int seed)
        {
            prng = new CSSystem.Random(seed);
        }

        public int Range(int minInclusive, int maxInclusive)
        {
            return prng.Next(minInclusive, maxInclusive);
        }

        public float Range(float minInclusive, float maxInclusive)
        {
            return (float)Range((double)minInclusive, (double)maxInclusive);
        }

        public double Range(double minInclusive, double maxInclusive)
        {
            var randomDouble = prng.NextDouble();
            var randomRangedDouble = randomDouble * (maxInclusive - minInclusive) + minInclusive;
            return randomRangedDouble;
        }

        public Color Color()
        {
            return Color(1.0f);
        }

        public Color Color(float normalizedAlpha)
        {
            return new Color(Range(0.0f, 1.0f), Range(0.0f, 1.0f), Range(0.0f, 1.0f), normalizedAlpha);
        }
    }
}
