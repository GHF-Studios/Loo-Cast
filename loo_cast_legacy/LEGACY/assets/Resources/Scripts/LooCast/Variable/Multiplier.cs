using System.Timers;
using UnityEngine.Events;

namespace LooCast.Variable
{
    public class Multiplier
    {
        public float Value { get; private set; }

        public Multiplier(float multiplier)
        {
            Value = multiplier;
        }
    }
}
