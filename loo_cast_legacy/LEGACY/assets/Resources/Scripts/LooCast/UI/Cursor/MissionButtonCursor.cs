using System;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Cursor
{
    using LooCast.UI.Button;
    using LooCast.Util;

    public class MissionButtonCursor : MonoBehaviour
    {
        public MissionButton CurrentMissionButton
        {
            get
            {
                return currentMissionButton;
            }

            set
            {
                currentMissionButton = value;
                transform.SetParent(CurrentMissionButton.transform, false);
                transform.SetAsLastSibling();
                missionRarityBorderImage.color = ColorUtil.RarityColors.GetMissionRarityColor(CurrentMissionButton.Mission.MissionRarity);
            }
        }

        [SerializeField] private Image missionRarityBorderImage;

        private MissionButton currentMissionButton;
    }
}
