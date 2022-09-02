using System;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Reward
{
    using LooCast.Mission;

    public class ReputationMissionReward : MissionReward
    {
        public void Initialize(LooCast.Mission.ReputationMissionReward reputationMissionReward)
        {
            rewardText.text = reputationMissionReward.ReputationReward.ToString("+#;-#;0") + " REP";
        }
    }
}