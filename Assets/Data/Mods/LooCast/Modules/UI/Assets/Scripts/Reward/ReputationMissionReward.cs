using System;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Reward
{
    using LooCast.Mission.Reward;

    public class ReputationMissionReward : MissionReward
    {
        public void Initialize(Mission.Reward.ReputationMissionReward reputationMissionReward, Color rarityColor)
        {
            rewardText.text = reputationMissionReward.ReputationReward.ToString("+#;-#;0") + " REP";
            SetRarityColor(rarityColor);
        }
    }
}