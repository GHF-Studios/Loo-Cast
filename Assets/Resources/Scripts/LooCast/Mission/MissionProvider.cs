using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Mission
{
    using Data;
    using LooCast.Util;
    
    public class MissionProvider : MonoBehaviour
    {
        [SerializeField] private MissionProviderData Data;

        public UnityEvent OnMissionListChange { get; private set; }

        public List<Mission> Missions
        {
            get
            {
                return missions;
            }
        }
        public List<Mission> CommonMissions
        {
            get
            {
                return Missions.Where((mission) => { return mission.MissionRarity == MissionRarity.Common; }).ToList();
            }
        }
        public List<Mission> UncommonMissions
        {
            get
            {
                return Missions.Where((mission) => { return mission.MissionRarity == MissionRarity.Uncommon; }).ToList();
            }
        }
        public List<Mission> RareMissions
        {
            get
            {
                return Missions.Where((mission) => { return mission.MissionRarity == MissionRarity.Rare; }).ToList();
            }
        }
        public List<Mission> EpicMissions
        {
            get
            {
                return Missions.Where((mission) => { return mission.MissionRarity == MissionRarity.Epic; }).ToList();
            }
        }
        public List<Mission> LegendaryMissions
        {
            get
            {
                return Missions.Where((mission) => { return mission.MissionRarity == MissionRarity.Legendary; }).ToList();
            }
        }
        public int Reputation
        {
            get
            {
                return reputation;
            }

            set
            {
                reputation = value;
            }
        }

        [SerializeField] private List<Mission> missions;
        private int reputation;

        private void Start()
        {
            if (Data.CommonMissionDatas.Length <= 0)
            {
                throw new Exception("Mission Provider Data can not contain less than 1 Common Mission Data!");
            }
            if (Data.UncommonMissionDatas.Length <= 0)
            {
                throw new Exception("Mission Provider Data can not contain less than 1 Uncommon Mission Data!");
            }
            if (Data.RareMissionDatas.Length <= 0)
            {
                throw new Exception("Mission Provider Data can not contain less than 1 Rare Mission Data!");
            }
            if (Data.EpicMissionDatas.Length <= 0)
            {
                throw new Exception("Mission Provider Data can not contain less than 1 Epic Mission Data!");
            }
            if (Data.LegendaryMissionDatas.Length <= 0)
            {
                throw new Exception("Mission Provider Data can not contain less than 1 Legendary Mission Data!");
            }

            OnMissionListChange = new UnityEvent();

            Reputation = Data.BaseReputation.Value;

            RefreshMissionList();
        }

        private void RefreshMissionList()
        {
            if (missions == null)
            {
                missions = new List<Mission>();
            }
            if (missions.Count < Data.MinMissionCount.Value)
            {
                GenerateMissions();
                OnMissionListChange.Invoke();
            }
        }

        private void GenerateMissions()
        {
            float commonMissionWeight = Data.CommonMissionWeight.Value * UnityEngine.Random.Range(Data.MinDeviationScale.Value, Data.MaxDeviationScale.Value);
            float uncommonMissionWeight = Data.UncommonMissionWeight.Value * UnityEngine.Random.Range(Data.MinDeviationScale.Value, Data.MaxDeviationScale.Value);
            float rareMissionWeight = Data.RareMissionWeight.Value * UnityEngine.Random.Range(Data.MinDeviationScale.Value, Data.MaxDeviationScale.Value);
            float epicMissionWeight = Data.EpicMissionWeight.Value * UnityEngine.Random.Range(Data.MinDeviationScale.Value, Data.MaxDeviationScale.Value);
            float legendaryMissionWeight = Data.LegendaryMissionWeight.Value * UnityEngine.Random.Range(Data.MinDeviationScale.Value, Data.MaxDeviationScale.Value);

            float missionWeightsSum = 0.0f;
            missionWeightsSum += commonMissionWeight;
            missionWeightsSum += uncommonMissionWeight;
            missionWeightsSum += rareMissionWeight;
            missionWeightsSum += epicMissionWeight;
            missionWeightsSum += legendaryMissionWeight;

            float commonMissionFraction = commonMissionWeight / missionWeightsSum;
            float uncommonMissionFraction = uncommonMissionWeight / missionWeightsSum;
            float rareMissionFraction = rareMissionWeight / missionWeightsSum;
            float epicMissionFraction = epicMissionWeight / missionWeightsSum;
            float legendaryMissionFraction = legendaryMissionWeight / missionWeightsSum;

            int commonMissionCount = Mathf.FloorToInt(Data.MaxMissionCount.Value * commonMissionFraction);
            int uncommonMissionCount = Mathf.FloorToInt(Data.MaxMissionCount.Value * uncommonMissionFraction);
            int rareMissionCount = Mathf.FloorToInt(Data.MaxMissionCount.Value * rareMissionFraction);
            int epicMissionCount = Mathf.FloorToInt(Data.MaxMissionCount.Value * epicMissionFraction);
            int legendaryMissionCount = Mathf.FloorToInt(Data.MaxMissionCount.Value * legendaryMissionFraction);

            int missionCountsSum = 0;
            missionCountsSum += commonMissionCount;
            missionCountsSum += uncommonMissionCount;
            missionCountsSum += rareMissionCount;
            missionCountsSum += epicMissionCount;
            missionCountsSum += legendaryMissionCount;

            for (int i = 0; i < commonMissionCount; i++)
            {
                if (CommonMissions.Count < commonMissionCount)
                {
                    int randomCommonMissionDataIndex = UnityEngine.Random.Range(0, Data.CommonMissionDatas.Length - 1);
                    MissionData randomCommonMissionData = Data.CommonMissionDatas[randomCommonMissionDataIndex];
                    missions.Add(randomCommonMissionData.CreateMission(this));
                }
                else
                {
                    break;
                }
            }

            for (int i = 0; i < uncommonMissionCount; i++)
            {
                if (UncommonMissions.Count < uncommonMissionCount)
                {
                    int randomUncommonMissionDataIndex = UnityEngine.Random.Range(0, Data.UncommonMissionDatas.Length - 1);
                    MissionData randomUncommonMissionData = Data.UncommonMissionDatas[randomUncommonMissionDataIndex];
                    missions.Add(randomUncommonMissionData.CreateMission(this));
                }
                else
                {
                    break;
                }
            }

            for (int i = 0; i < rareMissionCount; i++)
            {
                if (RareMissions.Count < rareMissionCount)
                {
                    int randomRareMissionDataIndex = UnityEngine.Random.Range(0, Data.RareMissionDatas.Length - 1);
                    MissionData randomRareMissionData = Data.RareMissionDatas[randomRareMissionDataIndex];
                    missions.Add(randomRareMissionData.CreateMission(this));
                }
                else
                {
                    break;
                }
            }

            for (int i = 0; i < epicMissionCount; i++)
            {
                if (EpicMissions.Count < epicMissionCount)
                {
                    int randomEpicMissionDataIndex = UnityEngine.Random.Range(0, Data.EpicMissionDatas.Length - 1);
                    MissionData randomEpicMissionData = Data.EpicMissionDatas[randomEpicMissionDataIndex];
                    missions.Add(randomEpicMissionData.CreateMission(this));
                }
                else
                {
                    break;
                }
            }

            for (int i = 0; i < legendaryMissionCount; i++)
            {
                if (LegendaryMissions.Count < legendaryMissionCount)
                {
                    int randomLegendaryMissionDataIndex = UnityEngine.Random.Range(0, Data.LegendaryMissionDatas.Length - 1);
                    MissionData randomLegendaryMissionData = Data.LegendaryMissionDatas[randomLegendaryMissionDataIndex];
                    missions.Add(randomLegendaryMissionData.CreateMission(this));
                }
                else
                {
                    break;
                }
            }

            missions.Shuffle();
        }

        public bool CanProvideMission(Mission mission)
        {
            if (!ContainsMission(mission))
            {
                throw new ArgumentException($"Mission with ID '{mission.ID}' is not contained in this Mission Provider!");
            }

            if (Reputation >= mission.RequiredReputation)
            {
                return true;
            }
            return false;
        }

        public bool ContainsMission(Mission checkMission)
        {
            foreach (Mission mission in missions)
            {
                if (checkMission == mission)
                {
                    return true;
                }
            }
            return false;
        }

        public void CompleteMission(Mission mission)
        {
            missions.Remove(mission);
            OnMissionListChange.Invoke();
        }
    }
}