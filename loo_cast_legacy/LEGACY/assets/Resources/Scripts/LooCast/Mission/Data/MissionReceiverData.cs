using UnityEngine;

namespace LooCast.Mission.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "MissionReceiverData", menuName = "Data/Mission/MissionReceiverData", order = 0)]
    public class MissionReceiverData : ScriptableObject
    {
        public IntDataReference MaxMissions;
        public IntDataReference MaxCommonMissions;
        public IntDataReference MaxUncommonMissions;
        public IntDataReference MaxRareMissions;
        public IntDataReference MaxEpicMissions;
        public IntDataReference MaxLegendaryMissions;
    }
}