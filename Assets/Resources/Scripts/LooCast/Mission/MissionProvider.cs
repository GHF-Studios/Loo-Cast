using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Mission
{
    using Data;
    
    public class MissionProvider : MonoBehaviour
    {
        [SerializeField] private MissionProviderData Data;

        [SerializeField] private List<Mission> missions;

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

        private int reputation;

        private void Start()
        {
            Reputation = Data.BaseReputation;
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
    }
}