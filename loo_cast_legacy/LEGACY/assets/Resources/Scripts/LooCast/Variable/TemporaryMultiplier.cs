using UnityEngine.Events;

namespace LooCast.Variable
{
    using Util;

    public class TemporaryMultiplier : Multiplier
    {
        public TimerUtil.Timer Timer { get; private set; }
        public UnityEvent OnTimerElapsed { get; private set; }

        public TemporaryMultiplier(float multiplier, float duration) : base(multiplier)
        {
            OnTimerElapsed = new UnityEvent();

            Timer = TimerUtil.CreateTimer(duration, true);
            Timer.AddElapsedAction(() => { OnTimerElapsed.Invoke(); });
        }
    }
}
