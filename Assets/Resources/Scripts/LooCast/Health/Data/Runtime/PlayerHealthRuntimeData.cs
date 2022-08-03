using UnityEngine;

namespace LooCast.Health.Data.Runtime
{
    using LooCast.Core.Data.Runtime;
    using LooCast.Stat;

    [CreateAssetMenu(fileName = "PlayerHealthRuntimeData", menuName = "Data/Health/Runtime/PlayerHealthRuntimeData", order = 0)]
    public class PlayerHealthRuntimeData : UniqueComponentRuntimeData
    {
        public float Health;
        public FloatStat MaxHealth;
        public FloatStat RegenerationAmount;
        public FloatStat RegenerationTime;
        public float RegenerationTimer;
        public IntStat Defense;
        public bool IsAlive;
    }
}
