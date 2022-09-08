using System;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Reward
{
    public class MissionReward : MonoBehaviour
    {
        [SerializeField] protected Text rewardText;
        [SerializeField] private Image[] missionRarityImages;

        protected void SetRarityColor(Color rarityColor)
        {
            foreach (Image missionRarityBorderImage in missionRarityImages)
            {
                missionRarityBorderImage.color = rarityColor;
            }
        }
    }
}