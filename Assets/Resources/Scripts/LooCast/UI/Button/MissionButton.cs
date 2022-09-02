using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Button
{
    using LooCast.Mission;
    using LooCast.Util;

    public class MissionButton : Button
    {
        public Mission Mission { get; private set; }

        [SerializeField] private Image missionRarityBorderImage;
        [SerializeField] private Text missionTitle;

        public void Initialize(Mission mission)
        {
            Mission = mission;
            missionRarityBorderImage.color = ColorUtil.RarityColors.GetMissionRarityColor(mission.MissionRarity);
            missionTitle.text = mission.MissionTitle;
        }

        public override void OnClick()
        {
            
        }
    }
}
