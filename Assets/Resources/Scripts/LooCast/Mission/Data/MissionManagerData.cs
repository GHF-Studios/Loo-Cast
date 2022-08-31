using UnityEngine;

namespace LooCast.Mission.Data
{
    using LooCast.Data;

    public class MissionManagerData : ScriptableObject
    {
        public IntDataReference MaxCommonMissions;
        public IntDataReference MaxUncommonMissions;
        public IntDataReference MaxRareMissions;
        public IntDataReference MaxEpicMissions;
        public IntDataReference MaxLegendaryMissions;
    }
}