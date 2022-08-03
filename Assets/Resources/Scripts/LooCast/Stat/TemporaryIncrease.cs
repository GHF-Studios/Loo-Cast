using System.Timers;
using UnityEngine.Events;

namespace LooCast.Stat
{
    public class TemporaryIncrease : Increase
    {
        public Timer timer { get; private set; }
        public UnityEvent OnTimerElapsed { get; private set; }

        public TemporaryIncrease(int increase, float duration) : base(increase)
        {
            timer = new Timer(duration * 1000);
            OnTimerElapsed = new UnityEvent();
            timer.Elapsed += (sender, args) => { OnTimerElapsed.Invoke(); };
        }
    }
}
