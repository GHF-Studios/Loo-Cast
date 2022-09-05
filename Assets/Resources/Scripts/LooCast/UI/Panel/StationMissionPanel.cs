using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Panel
{
    using LooCast.Mission;
    using LooCast.UI.Button;
    using LooCast.UI.Reward;
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
                    missionButton.UnityButton.onClick.AddListener(() =>
                    {
                        SelectedMission = missionButton.Mission;
                    });
                }
                SelectedMission = null;
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
                selectedMission = value;
                if (selectedMission == null)
                {
                    foreach (Image missionRarityBorderImage in missionRarityBorderImages)
                    {
                        missionRarityBorderImage.color = ColorUtil.RarityColors.GetMissionRarityColor(MissionRarity.Common);
                    }

                    missionTitle.enabled = false;
                    missionDescription.enabled = false;
                    missionTasks.enabled = false;
                    missionTasksTitle.enabled = false;
                    missionRewardTitle.enabled = false;
                    acceptMissionButton.enabled = false;
                    for (int i = 0; i < missionRewardParent.childCount; i++)
                    {
                        Destroy(missionRewardParent.GetChild(i).gameObject);
                    }
                }
                else
                {
                    Color rarityColor = ColorUtil.RarityColors.GetMissionRarityColor(selectedMission.MissionRarity);
                    foreach (Image missionRarityBorderImage in missionRarityBorderImages)
                    {
                        missionRarityBorderImage.color = rarityColor;
                    }

                    missionTitle.enabled = true;
                    missionDescription.enabled = true;
                    missionTasks.enabled = true;
                    missionTasksTitle.enabled = true;
                    missionRewardTitle.enabled = true;
                    acceptMissionButton.enabled = true;

                    missionTitle.text = selectedMission.MissionTitle;
                    missionDescription.text = selectedMission.MissionDescription;
                    missionTasks.text = selectedMission.MissionTasks;
                    for (int i = 0; i < missionRewardParent.childCount; i++)
                    {
                        Destroy(missionRewardParent.GetChild(i).gameObject);
                    }

                    foreach (LooCast.Mission.MissionReward missionReward in selectedMission.MissionRewards)
                    {
                        if (missionReward is LooCast.Mission.CreditsMissionReward)
                        {
                            GameObject creditsMissionRewardObject = Instantiate(creditsMissionRewardPrefab, missionRewardParent);
                            UI.Reward.CreditsMissionReward creditsMissionReward = creditsMissionRewardObject.GetComponent<UI.Reward.CreditsMissionReward>();
                            creditsMissionReward.Initialize((LooCast.Mission.CreditsMissionReward)missionReward);
                        }
                        else if (missionReward is LooCast.Mission.ReputationMissionReward)
                        {
                            GameObject reputationMissionRewardObject = Instantiate(reputationMissionRewardPrefab, missionRewardParent);
                            UI.Reward.ReputationMissionReward reputationMissionReward = reputationMissionRewardObject.GetComponent<UI.Reward.ReputationMissionReward>();
                            reputationMissionReward.Initialize((LooCast.Mission.ReputationMissionReward)missionReward);
                        }
                        else if (missionReward is LooCast.Mission.ItemMissionReward)
                        {
                            GameObject itemMissionRewardObject = Instantiate(itemMissionRewardPrefab, missionRewardParent);
                            UI.Reward.ItemMissionReward itemMissionReward = itemMissionRewardObject.GetComponent<UI.Reward.ItemMissionReward>();
                            itemMissionReward.Initialize((LooCast.Mission.ItemMissionReward)missionReward);
                        }
                    }
                }
            }
        }

        [SerializeField] private RectTransform missionButtonParent;
        [SerializeField] private GameObject missionButtonPrefab;
        [SerializeField] private Image[] missionRarityBorderImages;
        [SerializeField] private Text missionTitle;
        [SerializeField] private Text missionDescription;
        [SerializeField] private Text missionTasks;
        [SerializeField] private Text missionTasksTitle;
        [SerializeField] private Text missionRewardTitle;
        [SerializeField] private RectTransform missionRewardParent;
        [SerializeField] private GameObject creditsMissionRewardPrefab;
        [SerializeField] private GameObject reputationMissionRewardPrefab;
        [SerializeField] private GameObject itemMissionRewardPrefab;
        [SerializeField] private UnityEngine.UI.Button acceptMissionButton;
        
        private Mission selectedMission;
        private MissionProvider missionProvider;

        public void AcceptSelectedMission()
        {
            bool acceptedMission = MissionManager.Instance.TryAcceptMission(missionProvider, selectedMission);
            if (acceptedMission)
            {
                selectedMission = null;
            }
        }
    }
}