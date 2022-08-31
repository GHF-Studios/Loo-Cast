using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Mission
{
    using Data;

    public class MissionManager : MonoBehaviour
    {
        [SerializeField] private MissionManagerData data;

        public UnityEvent<Mission> OnActiveMissionChange { get; private set; }

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
                List<Mission> acceptedCommonMissions = new List<Mission>();
                foreach (Mission acceptedMission in acceptedMissions)
                {
                    if (acceptedMission.MissionRarity == MissionRarity.Common)
                    {
                        acceptedCommonMissions.Add(acceptedMission);
                    }
                }
                return acceptedCommonMissions;
            }
        }
        public List<Mission> AcceptedUncommonMissions
        {
            get
            {
                List<Mission> acceptedUncommonMissions = new List<Mission>();
                foreach (Mission acceptedMission in acceptedMissions)
                {
                    if (acceptedMission.MissionRarity == MissionRarity.Uncommon)
                    {
                        acceptedUncommonMissions.Add(acceptedMission);
                    }
                }
                return acceptedUncommonMissions;
            }
        }
        public List<Mission> AcceptedRareMissions
        {
            get
            {
                List<Mission> acceptedRareMissions = new List<Mission>();
                foreach (Mission acceptedMission in acceptedMissions)
                {
                    if (acceptedMission.MissionRarity == MissionRarity.Rare)
                    {
                        acceptedRareMissions.Add(acceptedMission);
                    }
                }
                return acceptedRareMissions;
            }
        }
        public List<Mission> AcceptedEpicMissions
        {
            get
            {
                List<Mission> acceptedEpicMissions = new List<Mission>();
                foreach (Mission acceptedMission in acceptedMissions)
                {
                    if (acceptedMission.MissionRarity == MissionRarity.Epic)
                    {
                        acceptedEpicMissions.Add(acceptedMission);
                    }
                }
                return acceptedEpicMissions;
            }
        }
        public List<Mission> AcceptedLegendaryMissions
        {
            get
            {
                List<Mission> acceptedLegendaryMissions = new List<Mission>();
                foreach (Mission acceptedMission in acceptedMissions)
                {
                    if (acceptedMission.MissionRarity == MissionRarity.Legendary)
                    {
                        acceptedLegendaryMissions.Add(acceptedMission);
                    }
                }
                return acceptedLegendaryMissions;
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

        private void Start()
        {
            OnActiveMissionChange = new UnityEvent<Mission>();
            
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

        public bool ContainsMission(Mission mission)
        {
            foreach (Mission acceptedMission in acceptedMissions)
            {
                if (acceptedMission == mission)
                {
                    return true;
                }
            }
            return false;
        }

        public bool TryAcceptMission(MissionProvider missionProvider, Mission mission)
        {
            if (ContainsMission(mission))
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
        }
    }
}