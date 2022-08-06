using System.Timers;
using UnityEngine.Events;

namespace LooCast.Variable
{
    public class TemporaryMultiplier : Multiplier
    {
        public Timer timer { get; private set; }
        public UnityEvent OnTimerElapsed { get; private set; }

        public TemporaryMultiplier(float multiplier, float duration) : base(multiplier)
        {
            timer = new Timer(duration * 1000);
            OnTimerElapsed = new UnityEvent();
            timer.Elapsed += (sender, args) => { OnTimerElapsed.Invoke(); };
        }
    }
}
