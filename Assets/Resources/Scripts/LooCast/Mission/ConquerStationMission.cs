using System;
using System.Collections.Generic;
using UnityEngine.Events;

namespace LooCast.Mission
{
    using Data;
    using Reward;
    using Target;
    using Task;
    using Trigger;
    using LooCast.Enemy;

    public class ConquerStationMission : Mission
    {
        public ConquerStationMission(ConquerStationMissionData data, MissionProvider missionProvider) : base(data, missionProvider)
        {
            AddReward(new CreditsMissionReward(data.Credits, data.CreditsReward.Value));
            AddReward(new ReputationMissionReward(missionProvider, data.ReputationReward.Value));
            AddReward(new ItemMissionReward(data.PlayerInventoryRuntimeData.Hotbar, data.ItemReward));

            KillXMissionTask rootTask = new KillXMissionTask(typeof(Enemy), data.RequiredEnemyKillCount.Value);
            InitializeRootTask(rootTask);
            

            //New Classes: LogicMissionTrigger(params MissionTrigger[] missionTriggers), LogicMissionTrigger(List<MissionTrigger> missionTriggers), ANDLogicMissionTrigger, ORLogicMissionTrigger
        }
    }
}