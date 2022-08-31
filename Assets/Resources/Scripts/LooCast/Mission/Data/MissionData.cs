using UnityEngine;

namespace LooCast.Mission.Data
{
    using LooCast.Data;

    public abstract class MissionData : ScriptableObject
    {
        public MissionRarity MissionRarity;
        public IntDataReference RequiredReputation;
    }
}