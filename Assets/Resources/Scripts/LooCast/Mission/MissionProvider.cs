using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Mission
{
    using Data;
    
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
            OnMissionListChange = new UnityEvent();

            Reputation = Data.BaseReputation.Value;

            RefreshMissionList();
        }

        private void RefreshMissionList()
        {
            if (missions == null || missions.Count < Data.MinMissionCount.Value)
            {
                int newMissionsCount = Data.MaxMissionCount.Value - missions.Count;
                GenerateMissions(newMissionsCount);
                OnMissionListChange.Invoke();
            }
        }

        private void GenerateMissions(int missionCount)
        {
            //First we get all the mission weights and calculate the mission weight sum
            float missionWeightSum = 0;
            float commonMissionWeight = Data.CommonMissionWeight.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f));
            float uncommonMissionWeight = Data.UncommonMissionWeight.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f));
            float rareMissionWeight = Data.RareMissionWeight.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f));
            float epicMissionWeight = Data.EpicMissionWeight.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f));
            float legendaryMissionWeight = Data.LegendaryMissionWeight.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f));
            missionWeightSum += commonMissionWeight;
            missionWeightSum += uncommonMissionWeight;
            missionWeightSum += rareMissionWeight;
            missionWeightSum += epicMissionWeight;
            missionWeightSum += legendaryMissionWeight;

            //Then we calculate the respective fractions of the mission weight sum
            float commonMissionFraction = commonMissionWeight / missionWeightSum;
            float uncommonMissionFraction = uncommonMissionWeight / missionWeightSum;
            float rareMissionFraction = rareMissionWeight / missionWeightSum;
            float epicMissionFraction = epicMissionWeight / missionWeightSum;
            float legendaryMissionFraction = legendaryMissionWeight / missionWeightSum;

            //Finally we actually create the missions
            if (missions == null)
            {
                missions = new List<Mission>();
            }
            for (int i = 0; i < missionCount; i++)
            {
                float randomEvaluation = UnityEngine.Random.Range(0.0f, 1.0f);
                if (randomEvaluation < commonMissionFraction)
                {
                    int randomCommonMissionDataIndex = UnityEngine.Random.Range(0, Data.CommonMissionDatas.Length - 1);
                    missions.Add(Data.CommonMissionDatas[i].CreateMission(this));
                }
                else if (randomEvaluation < uncommonMissionFraction)
                {
                    int randomUncommonMissionDataIndex = UnityEngine.Random.Range(0, Data.UncommonMissionDatas.Length - 1);
                    missions.Add(Data.UncommonMissionDatas[i].CreateMission(this));
                }
                else if (randomEvaluation < rareMissionFraction)
                {
                    int randomRareMissionDataIndex = UnityEngine.Random.Range(0, Data.RareMissionDatas.Length - 1);
                    missions.Add(Data.RareMissionDatas[i].CreateMission(this));
                }
                else if (randomEvaluation < epicMissionFraction)
                {
                    int randomEpicMissionDataIndex = UnityEngine.Random.Range(0, Data.EpicMissionDatas.Length - 1);
                    missions.Add(Data.EpicMissionDatas[i].CreateMission(this));
                }
                else if (randomEvaluation < legendaryMissionFraction)
                {
                    int randomLegendaryMissionDataIndex = UnityEngine.Random.Range(0, Data.LegendaryMissionDatas.Length - 1);
                    missions.Add(Data.LegendaryMissionDatas[i].CreateMission(this));
                }
            }
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