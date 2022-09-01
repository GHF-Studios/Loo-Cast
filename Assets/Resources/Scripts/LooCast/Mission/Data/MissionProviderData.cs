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
        public AnimationCurve CommonMissionWeight;
        public AnimationCurve UncommonMissionWeight;
        public AnimationCurve RareMissionWeight;
        public AnimationCurve EpicMissionWeight;
        public AnimationCurve LegendaryMissionWeight;
    }
}