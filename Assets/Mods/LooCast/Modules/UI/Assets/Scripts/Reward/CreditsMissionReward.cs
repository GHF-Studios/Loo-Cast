using System;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Reward
{
    using LooCast.Mission.Reward;

    public class CreditsMissionReward : MissionReward
    {
        public void Initialize(Mission.Reward.CreditsMissionReward creditsMissionReward, Color rarityColor)
        {
            rewardText.text = creditsMissionReward.CreditsReward.ToString("+#;-#;0") + " CR";
            SetRarityColor(rarityColor);
        }
    }
}