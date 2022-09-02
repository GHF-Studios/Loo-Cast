using System;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Reward
{
    using LooCast.Mission;

    public class CreditsMissionReward : MissionReward
    {
        public void Initialize(LooCast.Mission.CreditsMissionReward creditsMissionReward)
        {
            rewardText.text = creditsMissionReward.CreditsReward.ToString("+#;-#;0") + " CR";
        }
    }
}