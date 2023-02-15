using CSSystem = System;
using UnityEngine;

namespace LooCast.Chance
{
    public sealed class Chance
    {
        public Seed<CSSystem.IComparable> seed
        {
            get
            {
                return _seed;
            }

            private set
            {
                _seed = value;
            }
        }
        private Seed<CSSystem.IComparable> _seed;
        private AnimationCurve distribution;
        private CSSystem.Random random;

        public Chance(AnimationCurve distribution) : base()
        {
            seed = new Seed<int>(CSSystem.DateTime.Now.Millisecond);
            this.distribution = distribution;
            random = new CSSystem.Random((int)seed.seed);
        }

        public Chance(CSSystem.IComparable seed, AnimationCurve distribution) : base()
        {
            this.seed = new Seed<CSSystem.IComparable>(seed);
            this.distribution = distribution;
            if (seed is int || seed is float || seed is double)
            {
                random = new CSSystem.Random((int)seed);
            }
        }

        public float GetValue()
        {
            float value = (float)random.NextDouble();
            return distribution.Evaluate(value);
        }

        public static int GetRandomWeightedIndex(int[] weights)
        {
            // Get the total sum of all the weights.
            int weightSum = 0;
            foreach (int weight in weights)
            {
                weightSum += weight;
            }

            // Step through all the possibilities, one by one, checking to see if each one is selected.
            int index = 0;
            int lastIndex = weights.Length - 1;
            while (index < lastIndex)
            {
                // Do a probability check with a likelihood of weights[index] / weightSum.
                if (UnityEngine.Random.Range(0, weightSum) < weights[index])
                {
                    return index;
                }

                // Remove the last item from the sum of total untested weights and try again.
                weightSum -= weights[index++];
            }

            // No other item was selected, so return very last index.
            return index;
        }
    } 
}
