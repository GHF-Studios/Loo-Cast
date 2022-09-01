using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Mission
{
    using Data;

    public abstract class Mission
    {
        private static Dictionary<int, Mission> missionDictionary = new Dictionary<int, Mission>();
        private static int IDCounter = 0;
        public int ID { get; private set; }

        public UnityEvent OnAccept { get; private set; }
        public UnityEvent OnComplete { get; private set; }
        public UnityEvent OnFinalize { get; private set; }

        public MissionRarity MissionRarity { get; private set; } 
        public int RequiredReputation { get; private set; } 
        public MissionProvider MissionProvider { get; private set; }
        public MissionState MissionState
        {
            get
            {
                return missionState;
            }

            protected set
            {
                missionState = value;
            }
        }
        public string MissionTitle { get; private set; }
        public string MissionDescription { get; private set; }
        public string MissionTasks { get; private set; }
        public List<MissionReward> MissionRewards { get; private set; }

        [SerializeField] private MissionState missionState;

        public Mission(MissionData data, MissionProvider missionProvider)
        {
            ID = IDCounter;
            IDCounter++;

            OnAccept = new UnityEvent();
            OnComplete = new UnityEvent();
            OnFinalize = new UnityEvent();

            MissionRarity = data.MissionRarity;
            RequiredReputation = data.RequiredReputation.Value;
            MissionProvider = missionProvider;
            MissionState = MissionState.Offered;
            MissionTitle = data.MissionTitle.Value;
            MissionDescription = data.MissionDescription.Value;
            MissionTasks = data.MissionTasks.Value;
            MissionRewards = new List<MissionReward>();

            OnComplete.AddListener(() => { missionProvider.CompleteMission(this); });

            missionDictionary.Add(ID, this);
        }

        ~Mission()
        {
            OnFinalize.Invoke();
            missionDictionary.Remove(ID);
        }

        protected void AddReward(MissionReward reward)
        {
            MissionRewards.Add(reward);
        }

        public void Accept()
        {
            missionState = MissionState.Accepted;
            OnAccept.Invoke();
        }

        public void Complete()
        {
            missionState = MissionState.Completed;
            foreach (MissionReward missionReward in MissionRewards)
            {
                missionReward.Reward();
            }
            OnComplete.Invoke();
        }

        public override bool Equals(object obj)
        {
            Mission mission = (Mission)obj;
            if (mission != null && mission.ID == ID)
            {
                return true;
            }
            return false;
        }

        public override int GetHashCode()
        {
            return ID;
        }
    }
}