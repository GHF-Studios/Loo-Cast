using UnityEngine;

namespace LooCast.Health.Data
{
    using LooCast.Data;

    public abstract class HealthData : ScriptableObject
    {
        public FloatDataReference BaseMaxHealth;
        public FloatDataReference BaseRegenerationAmount;
        public FloatDataReference BaseRegenerationTime;
        public IntDataReference BaseDefense;
    } 
}
