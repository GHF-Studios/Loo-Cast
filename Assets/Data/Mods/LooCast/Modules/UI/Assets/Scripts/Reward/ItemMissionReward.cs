using System;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Reward
{
    using LooCast.Mission.Reward;

    public class ItemMissionReward : MissionReward
    {
        [SerializeField] private Image rewardImage;

        public void Initialize(Mission.Reward.ItemMissionReward itemMissionReward, Color rarityColor)
        {
            rewardText.text = itemMissionReward.RewardedItemData.ItemName.Value;
            rewardImage.sprite = itemMissionReward.RewardedItemData.Sprite;
            SetRarityColor(rarityColor);
        }
    }
}