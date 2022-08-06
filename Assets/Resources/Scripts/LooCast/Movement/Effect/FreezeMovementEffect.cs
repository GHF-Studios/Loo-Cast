using UnityEngine;

namespace LooCast.Movement.Effect
{
    using LooCast.Variable;

    public class FreezeMovementEffect : MovementEffect
    {
        public void Initialize(float speedMultiplier, float duration)
        {
            TemporaryMultiplier multiplier = Movement.Speed.AddTimedMultiplier(speedMultiplier, duration);
            multiplier.OnTimerElapsed.AddListener(() => { Destroy(this); });
        }
    }
}
