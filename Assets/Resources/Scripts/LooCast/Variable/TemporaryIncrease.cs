using UnityEngine.Events;

namespace LooCast.Variable
{
    using Util;

    public class TemporaryIncrease : Increase
    {
        public TimerUtil.Timer Timer { get; private set; }
        public UnityEvent OnTimerElapsed { get; private set; }

        public TemporaryIncrease(int increase, float duration) : base(increase)
        {
            OnTimerElapsed = new UnityEvent();

            Timer = TimerUtil.CreateTimer(duration, true);
            Timer.AddElapsedAction(() => { OnTimerElapsed.Invoke(); });
        }
    }
}
