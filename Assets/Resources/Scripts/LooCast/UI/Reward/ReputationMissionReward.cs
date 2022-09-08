using System;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Reward
{
    public class ReputationMissionReward : MissionReward
    {
        public void Initialize(Mission.ReputationMissionReward reputationMissionReward, Color rarityColor)
        {
            rewardText.text = reputationMissionReward.ReputationReward.ToString("+#;-#;0") + " REP";
            SetRarityColor(rarityColor);
        }
    }
}