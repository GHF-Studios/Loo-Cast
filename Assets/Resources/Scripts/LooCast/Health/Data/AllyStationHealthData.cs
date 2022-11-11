using UnityEngine;

namespace LooCast.Health.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "AllyStationHealthData", menuName = "Data/Health/AllyStationHealthData", order = 0)]
    public class AllyStationHealthData : ScriptableObject
    {
        public FloatDataReference BaseMaxHealth;
        public FloatDataReference BaseRegenerationAmount;
        public FloatDataReference BaseRegenerationTime;
        public IntDataReference BaseDefense;
        public FloatDataReference BaseKnockbackResistance;
        public GameObject DamageIndicatorPrefab;
        public IHealth.TeamType Team;
    } 
}
