using UnityEngine;
using System.Collections.Generic;

namespace LooCast.Mission.Data
{
    using LooCast.Data;

    [CreateAssetMenu(fileName = "MissionProviderData", menuName = "Data/Mission/MissionProviderData", order = 0)]
    public class MissionProviderData : ScriptableObject
    {
        public IntDataReference BaseReputation;
        public IntDataReference MinMissionCount;
        public IntDataReference MaxMissionCount;
        public MissionData[] CommonMissionDatas;
        public MissionData[] UncommonMissionDatas;
        public MissionData[] RareMissionDatas;
        public MissionData[] EpicMissionDatas;
        public MissionData[] LegendaryMissionDatas;
        public FloatDataReference CommonMissionWeight;
        public FloatDataReference UncommonMissionWeight;
        public FloatDataReference RareMissionWeight;
        public FloatDataReference EpicMissionWeight;
        public FloatDataReference LegendaryMissionWeight;
        public FloatDataReference MinDeviationScale;
        public FloatDataReference MaxDeviationScale;
    }
}