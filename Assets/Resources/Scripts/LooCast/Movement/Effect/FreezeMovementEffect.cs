using UnityEngine;

namespace LooCast.Movement.Effect
{
    using LooCast.Stat;

    public class FreezeMovementEffect : MovementEffect
    {
        public void Initialize(float freezingSlowness)
        {
            TemporaryMultiplier multiplier = Movement.Speed.AddTimedMultiplier(0.75f, 10.0f);
            multiplier.OnTimerElapsed.AddListener(() => { Destroy(this); });
        }
    }
}
