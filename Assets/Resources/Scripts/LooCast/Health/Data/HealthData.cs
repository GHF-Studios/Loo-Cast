using UnityEngine;

namespace LooCast.Health.Data
{
    using LooCast.Data;

    public abstract class HealthData : ScriptableObject
    {
        public FloatReference BaseMaxHealth;
        public FloatReference BaseRegenerationAmount;
        public FloatReference BaseRegenerationTime;
        public IntReference BaseDefense;
    } 
}
