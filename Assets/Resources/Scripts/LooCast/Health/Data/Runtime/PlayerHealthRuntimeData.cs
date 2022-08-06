using UnityEngine;

namespace LooCast.Health.Data.Runtime
{
    using LooCast.Variable;

    [CreateAssetMenu(fileName = "PlayerHealthRuntimeData", menuName = "Data/Health/Runtime/PlayerHealthRuntimeData", order = 0)]
    public class PlayerHealthRuntimeData : ScriptableObject
    {
        public FloatVariable Health;
        public FloatComputedVariable MaxHealth;
        public FloatComputedVariable RegenerationAmount;
        public FloatComputedVariable RegenerationTime;
        public FloatVariable RegenerationTimer;
        public IntComputedVariable Defense;
        public BoolVariable IsAlive;
    }
}
