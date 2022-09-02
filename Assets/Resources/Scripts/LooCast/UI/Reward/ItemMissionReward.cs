using System;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Reward
{
    using LooCast.Item;

    public class ItemMissionReward : MissionReward
    {
        [SerializeField] private Image rewardImage;

        private LooCast.Mission.ItemMissionReward itemMissionReward
        {
            set
            {
                if (value == null)
                {
                    rewardText.text = "null";
                    rewardImage.sprite = null;
                }
                else
                {
                    rewardText.text = value.RewardedItem.Name;
                    rewardImage.sprite = value.RewardedItem.Sprite;
                }
            }
        }

        public void Initialize(LooCast.Mission.ItemMissionReward itemMissionReward)
        {
            this.itemMissionReward = itemMissionReward;
        }
    }
}