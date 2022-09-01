using UnityEngine;

namespace LooCast.Mission.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "MissionManagerData", menuName = "Data/Mission/MissionManagerData", order = 0)]
    public class MissionManagerData : ScriptableObject
    {
        public IntDataReference MaxCommonMissions;
        public IntDataReference MaxUncommonMissions;
        public IntDataReference MaxRareMissions;
        public IntDataReference MaxEpicMissions;
        public IntDataReference MaxLegendaryMissions;
    }
}