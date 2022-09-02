using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;
using DataStructures.RandomSelector;

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
                int newMissionsCount = Data.MaxMissionCount.Value - missions.Count;
                GenerateMissions(newMissionsCount);
                OnMissionListChange.Invoke();
            }
        }

        private void GenerateMissions(int missionCount)
        {
            float commonMissionWeight = Data.CommonMissionWeight.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f));
            float uncommonMissionWeight = Data.UncommonMissionWeight.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f));
            float rareMissionWeight = Data.RareMissionWeight.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f));
            float epicMissionWeight = Data.EpicMissionWeight.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f));
            float legendaryMissionWeight = Data.LegendaryMissionWeight.Evaluate(UnityEngine.Random.Range(0.0f, 1.0f));

            DynamicRandomSelector<MissionData[]> dynamicRandomSelector = new DynamicRandomSelector<MissionData[]>();
            dynamicRandomSelector.Add(Data.CommonMissionDatas, commonMissionWeight);
            dynamicRandomSelector.Add(Data.UncommonMissionDatas, uncommonMissionWeight);
            dynamicRandomSelector.Add(Data.RareMissionDatas, rareMissionWeight);
            dynamicRandomSelector.Add(Data.EpicMissionDatas, epicMissionWeight);
            dynamicRandomSelector.Add(Data.LegendaryMissionDatas, legendaryMissionWeight);
            dynamicRandomSelector.Build();
            if (missions == null)
            {
                missions = new List<Mission>();
            }
            for (int i = 0; i < missionCount; i++)
            {
                MissionData[] randomMissionDatas = dynamicRandomSelector.SelectRandomItem();
                int randomMissionDataIndex = UnityEngine.Random.Range(0, randomMissionDatas.Length - 1);
                MissionData randomMissionData = randomMissionDatas[randomMissionDataIndex];
                missions.Add(randomMissionData.CreateMission(this));
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