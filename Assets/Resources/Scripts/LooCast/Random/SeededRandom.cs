using UnityEngine;
using System;

namespace LooCast.Random
{
    public class SeededRandom
    {
        private System.Random prng;

        public SeededRandom(int seed)
        {
            prng = new System.Random(seed);
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
            return new Color(Range(0.0f, 1.0f), Range(0.0f, 1.0f), Range(0.0f, 1.0f), 1.0f);
        }
    }
}
