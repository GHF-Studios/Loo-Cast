using UnityEngine;
using System.Collections.Generic;

namespace LooCast.Mission.Data
{
    using Data;
    
    public class MissionProviderData : ScriptableObject
    {
        public int BaseReputation;
        public int MinMissionCount;
        public int MaxMissionCount;
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