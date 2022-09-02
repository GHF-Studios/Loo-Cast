using System;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Reward
{
    using LooCast.Item;

    public class ItemReward : MonoBehaviour
    {
        [SerializeField] private Text rewardText;
        [SerializeField] private Reward rewardImage;

        private Item item
        {
            set
            {
                if (value == null)
                {
                    rewardText.text = "null";
                    rewardImage.image = null;
                }
                else
                {
                    rewardText.text = value.Name;
                    rewardImage.image = value.Sprite;
                }
            }
        }

        public void Initialize(Item item)
        {
            this.item = item;
        }
    }
}