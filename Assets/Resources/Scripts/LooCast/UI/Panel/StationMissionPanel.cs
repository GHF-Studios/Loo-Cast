using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Panel
{
    using LooCast.Mission;
    using LooCast.UI.Button;
    using LooCast.Util;

    public class StationMissionPanel : Panel
    {
        public MissionProvider MissionProvider
        {
            get
            {
                return missionProvider;
            }

            set
            {
                missionProvider = value;
                if (missionProvider.Missions.Count == 0)
                {
                    throw new Exception("Mission Provider must contain at least 1 Mission!");
                }

                for (int i = 0; i < missionButtonParent.childCount; i++)
                {
                    Destroy(missionButtonParent.GetChild(i).gameObject);
                }

                foreach (Mission mission in missionProvider.Missions)
                {
                    GameObject missionButtonObject = Instantiate(missionButtonPrefab, missionButtonParent);
                    MissionButton missionButton = missionButtonObject.GetComponent<MissionButton>();
                    missionButton.Initialize(mission);
                }
                SelectedMission = missionProvider.Missions[0];
            }
        }
        public Mission SelectedMission
        {
            get
            {
                return selectedMission;
            }

            private set
            {
                if (value == null)
                {
                    throw new ArgumentNullException("Selected Mission can not be null!");
                }
                selectedMission = value;

                Color rarityColor = ColorUtil.RarityColors.GetMissionRarityColor(selectedMission.MissionRarity);
                foreach (Image missionRarityBorderImage in missionRarityBorderImages)
                {
                    missionRarityBorderImage.color = rarityColor;
                }

                missionTitle.text = selectedMission.MissionTitle;
                missionDescription.text = selectedMission.MissionDescription;
                missionTasks.text = selectedMission.MissionTasks;
                for (int i = 0; i < missionRewardParent.childCount; i++)
                {
                    Destroy(missionRewardParent.GetChild(i).gameObject);
                }
                foreach (MissionReward missionReward in selectedMission.MissionRewards)
                {
                    GameObject missionRewardObject = Instantiate(missionRewardPrefab, missionRewardParent);
                     = missionRewardObject.GetComponent<>();
                    .Initialize();
                }
            }
        }

        [SerializeField] private RectTransform missionButtonParent;
        [SerializeField] private GameObject missionButtonPrefab;
        [SerializeField] private Image[] missionRarityBorderImages;
        [SerializeField] private Text missionTitle;
        [SerializeField] private Text missionDescription;
        [SerializeField] private Text missionTasks;
        [SerializeField] private RectTransform missionRewardParent;
        [SerializeField] private GameObject missionRewardPrefab;
        
        private Mission selectedMission;
        private MissionProvider missionProvider;

        public void Refresh()
        {
            
        }
    }
}