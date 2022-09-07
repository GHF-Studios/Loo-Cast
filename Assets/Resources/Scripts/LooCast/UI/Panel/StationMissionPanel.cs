using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Panel
{
    using LooCast.Mission;
    using LooCast.UI.Button;
    using LooCast.UI.Cursor;
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
                if (missionProvider == null)
                {
                    throw new NullReferenceException("Mission Provider can not be null!");
                }
                if (missionProvider.Missions.Count == 0)
                {
                    throw new Exception("Mission Provider must contain at least 1 Mission!");
                }

                missionProvider.OnMissionListChange.AddListener(() => { RefreshMissionList(); });
                RefreshMissionList();
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
                    if (missionButtonCursor != null)
                    {
                        Destroy(missionButtonCursor.gameObject);
                    }
                    
                    foreach (Image missionRarityBorderImage in missionRarityBorderImages)
                    {
                        missionRarityBorderImage.color = ColorUtil.RarityColors.GetMissionRarityColor(MissionRarity.Common);
                    }

                    missionTitle.gameObject.SetActive(false);
                    missionDescription.gameObject.SetActive(false);
                    missionTasks.gameObject.SetActive(false);
                    missionTasksTitle.gameObject.SetActive(false);
                    missionRewardTitle.gameObject.SetActive(false);
                    missionAcceptButton.interactable = false;
                    acceptedRarityMissionsLabel.gameObject.SetActive(false);
                    acceptedRarityMissionsValue.gameObject.SetActive(false);
                    acceptedTotalMissionsLabel.gameObject.SetActive(false);
                    acceptedTotalMissionsValue.gameObject.SetActive(false);
                    for (int i = 0; i < missionRewardParent.childCount; i++)
                    {
                        Destroy(missionRewardParent.GetChild(i).gameObject);
                    }
                }
                else
                {
                    Color rarityColor = ColorUtil.RarityColors.GetMissionRarityColor(selectedMission.MissionRarity);

                    if (missionButtonCursor == null)
                    {
                        GameObject missionButtonCursorObject = Instantiate(missionButtonCursorPrefab);
                        missionButtonCursor = missionButtonCursorObject.GetComponent<MissionButtonCursor>();
                    }

                    foreach (Image missionRarityBorderImage in missionRarityBorderImages)
                    {
                        missionRarityBorderImage.color = rarityColor;
                    }

                    missionTitle.gameObject.SetActive(true);
                    missionDescription.gameObject.SetActive(true);
                    missionTasks.gameObject.SetActive(true);
                    missionTasksTitle.gameObject.SetActive(true);
                    missionRewardTitle.gameObject.SetActive(true);
                    missionAcceptButton.interactable = true;
                    acceptedRarityMissionsLabel.gameObject.SetActive(true);
                    acceptedRarityMissionsValue.gameObject.SetActive(true);
                    acceptedTotalMissionsLabel.gameObject.SetActive(true);
                    acceptedTotalMissionsValue.gameObject.SetActive(true);

                    missionTitle.text = selectedMission.MissionTitle;
                    missionDescription.text = selectedMission.MissionDescription;
                    missionTasks.text = selectedMission.MissionTasks;
                    missionAcceptButton.onClick.AddListener(() =>
                    {
                        bool acceptMissionSuccess = MissionManager.Instance.TryAcceptMission(missionProvider, selectedMission);
                        if (acceptMissionSuccess)
                        {
                            selectedMissionButton.transform.SetParent(acceptedMissionsParent, false);
                        }
                        else
                        {
                            throw new InvalidOperationException("Could not accept Mission!");
                        }
                    });
                    int acceptedMissionCount;
                    int maxAcceptedMissionCount;
                    switch (selectedMission.MissionRarity)
                    {
                        case MissionRarity.Common:
                            acceptedMissionCount = MissionManager.Instance.AcceptedCommonMissions.Count;
                            maxAcceptedMissionCount = MissionManager.Instance.MaxCommonMissions;
                            break;
                        case MissionRarity.Uncommon:
                            acceptedMissionCount = MissionManager.Instance.AcceptedUncommonMissions.Count;
                            maxAcceptedMissionCount = MissionManager.Instance.MaxUncommonMissions;
                            break;
                        case MissionRarity.Rare:
                            acceptedMissionCount = MissionManager.Instance.AcceptedRareMissions.Count;
                            maxAcceptedMissionCount = MissionManager.Instance.MaxRareMissions;
                            break;
                        case MissionRarity.Epic:
                            acceptedMissionCount = MissionManager.Instance.AcceptedEpicMissions.Count;
                            maxAcceptedMissionCount = MissionManager.Instance.MaxEpicMissions;
                            break;
                        case MissionRarity.Legendary:
                            acceptedMissionCount = MissionManager.Instance.AcceptedLegendaryMissions.Count;
                            maxAcceptedMissionCount = MissionManager.Instance.MaxLegendaryMissions;
                            break;
                        default:
                            throw new Exception("Selected Mission: Invalid Mission Rarity!");
                    }

                    acceptedRarityMissionsLabel.text = $"{selectedMission.MissionRarity}:";
                    acceptedRarityMissionsValue.text = $"{acceptedMissionCount}/{maxAcceptedMissionCount}";
                    acceptedTotalMissionsValue.text = $"{MissionManager.Instance.AcceptedMissions.Count}/{MissionManager.Instance.MaxMissions}";

                    if (!MissionManager.Instance.CanAcceptMission(selectedMission) || selectedMission.MissionState != MissionState.Offered)
                    {
                        missionAcceptButton.interactable = false;
                    }
                    else
                    {
                        missionAcceptButton.interactable = true;
                    }

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

        [SerializeField] private RectTransform acceptedMissionsParent;
        [SerializeField] private RectTransform offeredMissionsParent;
        [SerializeField] private GameObject missionButtonPrefab;
        [SerializeField] private GameObject missionButtonCursorPrefab;
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
        [SerializeField] private UnityEngine.UI.Button missionAcceptButton;
        [SerializeField] private Text acceptedRarityMissionsLabel;
        [SerializeField] private Text acceptedRarityMissionsValue;
        [SerializeField] private Text acceptedTotalMissionsLabel;
        [SerializeField] private Text acceptedTotalMissionsValue;
        
        private Mission selectedMission;
        private MissionButton selectedMissionButton;
        private MissionProvider missionProvider;
        private MissionButtonCursor missionButtonCursor;

        public void RefreshMissionList()
        {
            for (int i = 1; i < acceptedMissionsParent.childCount; i++)
            {
                Destroy(acceptedMissionsParent.GetChild(i).gameObject);
            }
            for (int i = 1; i < offeredMissionsParent.childCount; i++)
            {
                Destroy(offeredMissionsParent.GetChild(i).gameObject);
            }

            foreach (Mission mission in missionProvider.AcceptedMissions)
            {
                GameObject missionButtonObject = Instantiate(missionButtonPrefab, acceptedMissionsParent);
                MissionButton missionButton = missionButtonObject.GetComponent<MissionButton>();
                missionButton.Initialize(mission);
                missionButton.UnityButton.onClick.AddListener(() =>
                {
                    SelectedMission = missionButton.Mission;
                    selectedMissionButton = missionButton;
                    missionButtonCursor.CurrentMissionButton = missionButton;

                });
            }
            foreach (Mission mission in missionProvider.OfferedMissions)
            {
                GameObject missionButtonObject = Instantiate(missionButtonPrefab, offeredMissionsParent);
                MissionButton missionButton = missionButtonObject.GetComponent<MissionButton>();
                missionButton.Initialize(mission);
                missionButton.UnityButton.onClick.AddListener(() =>
                {
                    SelectedMission = missionButton.Mission;
                    selectedMissionButton = missionButton;
                    missionButtonCursor.CurrentMissionButton = missionButton;
                });
            }
        }
    }
}