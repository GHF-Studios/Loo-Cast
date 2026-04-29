using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Mission
{
    using Data;

    public class MissionReceiver : MonoBehaviour
    {
        public static MissionReceiver Instance { get; private set;}

        [SerializeField] private MissionReceiverData data;

        public UnityEvent<Mission> OnActiveMissionChange { get; private set; }
        public UnityEvent<Mission> OnAcceptMission { get; private set; }

        public int MaxMissions { get; private set; }
        public int MaxCommonMissions { get; private set; }
        public int MaxUncommonMissions { get; private set; }
        public int MaxRareMissions { get; private set; }
        public int MaxEpicMissions { get; private set; }
        public int MaxLegendaryMissions { get; private set; }
        public List<Mission> AcceptedMissions
        {
            get
            {
                return acceptedMissions;
            }
        }
        public List<Mission> AcceptedCommonMissions
        {
            get
            {
                return AcceptedMissions.Where((mission) => { return mission.MissionRarity == MissionRarity.Common; }).ToList();
            }
        }
        public List<Mission> AcceptedUncommonMissions
        {
            get
            {
                return AcceptedMissions.Where((mission) => { return mission.MissionRarity == MissionRarity.Uncommon; }).ToList();
            }
        }
        public List<Mission> AcceptedRareMissions
        {
            get
            {
                return AcceptedMissions.Where((mission) => { return mission.MissionRarity == MissionRarity.Rare; }).ToList();
            }
        }
        public List<Mission> AcceptedEpicMissions
        {
            get
            {
                return AcceptedMissions.Where((mission) => { return mission.MissionRarity == MissionRarity.Epic; }).ToList();
            }
        }
        public List<Mission> AcceptedLegendaryMissions
        {
            get
            {
                return AcceptedMissions.Where((mission) => { return mission.MissionRarity == MissionRarity.Legendary; }).ToList();
            }
        }
        public List<Mission> CompletedMissions
        {
            get
            {
                return completedMissions;
            }
        }
        public Mission ActiveMission
        {
            get
            {
                return activeMission;
            }

            set
            {
                switch (value.MissionState)
                {
                    case MissionState.Offered:
                        throw new ArgumentException("Cannot set Mission active: Mission has not yet been accepted!");
                    case MissionState.Completed:
                        throw new ArgumentException("Cannot set Mission active: Mission has already been completed!");
                    default:
                        break;
                }
                activeMission = value;
                OnActiveMissionChange.Invoke(activeMission);
            }
        }

        private List<Mission> acceptedMissions;
        private List<Mission> completedMissions;
        private Mission activeMission;

        private void Awake()
        {
            if (Instance != null && Instance != this)
            {
                Destroy(gameObject);
            }
            else
            {
                Instance = this;
            }

            OnActiveMissionChange = new UnityEvent<Mission>();
            OnAcceptMission = new UnityEvent<Mission>();

            MaxMissions = data.MaxMissions.Value;
            MaxCommonMissions = data.MaxCommonMissions.Value;
            MaxUncommonMissions = data.MaxUncommonMissions.Value;
            MaxRareMissions = data.MaxRareMissions.Value;
            MaxEpicMissions = data.MaxEpicMissions.Value;
            MaxLegendaryMissions = data.MaxLegendaryMissions.Value;
            acceptedMissions = new List<Mission>();
            completedMissions = new List<Mission>();
            activeMission = null;
        }

        public bool CanAcceptMission(Mission mission)
        {
            if (AcceptedMissions.Count >= MaxMissions)
            {
                return false;
            }
            switch (mission.MissionRarity)
            {
                case MissionRarity.Common:
                    if (AcceptedCommonMissions.Count >= MaxCommonMissions)
                    {
                        return false;
                    }
                    return true;
                case MissionRarity.Uncommon:
                    if (AcceptedUncommonMissions.Count >= MaxUncommonMissions)
                    {
                        return false;
                    }
                    return true;
                case MissionRarity.Rare:
                    if (AcceptedRareMissions.Count >= MaxRareMissions)
                    {
                        return false;
                    }
                    return true;
                case MissionRarity.Epic:
                    if (AcceptedEpicMissions.Count >= MaxEpicMissions)
                    {
                        return false;
                    }
                    return true;
                case MissionRarity.Legendary:
                    if (AcceptedLegendaryMissions.Count >= MaxLegendaryMissions)
                    {
                        return false;
                    }
                    return true;
                default:
                    throw new NotImplementedException($"Mission Rarity '{mission.MissionRarity}' is not implemented!");
            }
        }

        public bool TryAcceptMission(MissionProvider missionProvider, Mission mission)
        {
            if (AcceptedMissions.Contains(mission))
            {
                throw new ArgumentException("Already accepted mission!");
            }
            
            if (!CanAcceptMission(mission) || !missionProvider.CanProvideMission(mission))
            {
                return false;
            }
            AcceptMission(mission);
            return true;
        }

        private void AcceptMission(Mission mission)
        {
            acceptedMissions.Add(mission);
            mission.Accept();
            ActiveMission = mission;
            OnAcceptMission.Invoke(mission);
        }
    }
}