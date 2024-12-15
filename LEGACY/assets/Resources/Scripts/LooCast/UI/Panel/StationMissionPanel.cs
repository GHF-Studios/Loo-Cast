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
    using LooCast.Mission.Reward;
    using LooCast.Mission.Task;

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
                    RefreshMissionButtonCursor();
                    RefreshMissionRewards();
                    SetRarityColor(ColorUtil.RarityColors.GetMissionRarityColor(MissionRarity.Common));
                    SetPanelContentVisibility(false);
                }
                else
                {
                    RefreshMissionTasks();
                    missionAcceptButton.onClick.RemoveAllListeners();
                    missionAcceptButton.onClick.AddListener(() =>
                    {
                        bool acceptMissionSuccess = MissionReceiver.Instance.TryAcceptMission(missionProvider, selectedMission);
                        if (acceptMissionSuccess)
                        {
                            selectedMissionButton.transform.SetParent(acceptedMissionsParent, false);
                        }
                        else
                        {
                            throw new InvalidOperationException("Could not accept Mission!");
                        }
                    });
                    MissionReceiver.Instance.OnAcceptMission.AddListener((mission) => 
                    { 
                        RefreshAcceptedMissionCount();
                        RefreshMissionAcceptButton();
                    });
                    RefreshMissionSumary();
                    RefreshMissionButtonCursor();
                    RefreshAcceptedMissionCount();
                    RefreshMissionAcceptButton();
                    RefreshMissionRewards();
                    SetRarityColor(ColorUtil.RarityColors.GetMissionRarityColor(selectedMission.MissionRarity));
                    SetPanelContentVisibility(true);
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
        [SerializeField] private Text missionRequiredReputationLabel;
        [SerializeField] private Text missionRequiredReputationValue;
        [SerializeField] private Text missionTasks;
        [SerializeField] private Text missionTasksTitle;
        [SerializeField] private Text missionRewardTitle;
        [SerializeField] private RectTransform missionRewardParent;
        [SerializeField] private GameObject creditsMissionRewardPrefab;
        [SerializeField] private GameObject reputationMissionRewardPrefab;
        [SerializeField] private GameObject itemMissionRewardPrefab;
        [SerializeField] private UnityEngine.UI.Button missionAcceptButton;
        [SerializeField] private Text missionAcceptButtonLabel;
        [SerializeField] private Text acceptedRarityMissionsLabel;
        [SerializeField] private Text acceptedRarityMissionsValue;
        [SerializeField] private Text acceptedTotalMissionsLabel;
        [SerializeField] private Text acceptedTotalMissionsValue;
        
        private Mission selectedMission;
        private MissionButton selectedMissionButton;
        private MissionProvider missionProvider;
        private MissionButtonCursor missionButtonCursor;

        private void RefreshMissionList()
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

        private void RefreshMissionSumary()
        {
            missionTitle.text = selectedMission.MissionTitle;
            missionDescription.text = selectedMission.MissionDescription;
            missionRequiredReputationValue.text = $"{selectedMission.RequiredReputation} REP";
        }

        private void RefreshAcceptedMissionCount()
        {
            int acceptedMissionCount;
            int maxAcceptedMissionCount;
            switch (selectedMission.MissionRarity)
            {
                case MissionRarity.Common:
                    acceptedMissionCount = MissionReceiver.Instance.AcceptedCommonMissions.Count;
                    maxAcceptedMissionCount = MissionReceiver.Instance.MaxCommonMissions;
                    break;
                case MissionRarity.Uncommon:
                    acceptedMissionCount = MissionReceiver.Instance.AcceptedUncommonMissions.Count;
                    maxAcceptedMissionCount = MissionReceiver.Instance.MaxUncommonMissions;
                    break;
                case MissionRarity.Rare:
                    acceptedMissionCount = MissionReceiver.Instance.AcceptedRareMissions.Count;
                    maxAcceptedMissionCount = MissionReceiver.Instance.MaxRareMissions;
                    break;
                case MissionRarity.Epic:
                    acceptedMissionCount = MissionReceiver.Instance.AcceptedEpicMissions.Count;
                    maxAcceptedMissionCount = MissionReceiver.Instance.MaxEpicMissions;
                    break;
                case MissionRarity.Legendary:
                    acceptedMissionCount = MissionReceiver.Instance.AcceptedLegendaryMissions.Count;
                    maxAcceptedMissionCount = MissionReceiver.Instance.MaxLegendaryMissions;
                    break;
                default:
                    throw new Exception("Selected Mission: Invalid Mission Rarity!");
            }

            acceptedRarityMissionsLabel.text = $"{selectedMission.MissionRarity}:";
            acceptedRarityMissionsValue.text = $"{acceptedMissionCount}/{maxAcceptedMissionCount}";
            acceptedTotalMissionsValue.text = $"{MissionReceiver.Instance.AcceptedMissions.Count}/{MissionReceiver.Instance.MaxMissions}";
        }

        private void RefreshMissionAcceptButton()
        {
            if (!MissionReceiver.Instance.CanAcceptMission(selectedMission) || selectedMission.MissionState != MissionState.Offered)
            {
                missionAcceptButton.interactable = false;
                if (selectedMission.MissionState == MissionState.Accepted)
                {
                    missionAcceptButtonLabel.text = "Mission already accepted!";
                }
            }
            else
            {
                if (missionProvider.CanProvideMission(selectedMission))
                {
                    missionAcceptButton.interactable = true;
                    missionAcceptButtonLabel.text = "Accept Mission";
                }
                else
                {
                    missionAcceptButton.interactable = false;
                    missionAcceptButtonLabel.text = "Need more Reputation!";
                }
            }
        }

        private void RefreshMissionTasks()
        {
            missionTasks.text = selectedMission.RootMissionTask.Summary;
        }

        private void RefreshMissionRewards()
        {
            for (int i = 0; i < missionRewardParent.childCount; i++)
            {
                Destroy(missionRewardParent.GetChild(i).gameObject);
            }

            if (selectedMission != null)
            {
                foreach (MissionReward missionReward in selectedMission.MissionRewards)
                {
                    if (missionReward is CreditsMissionReward)
                    {
                        GameObject creditsMissionRewardObject = Instantiate(creditsMissionRewardPrefab, missionRewardParent);
                        Reward.CreditsMissionReward creditsMissionReward = creditsMissionRewardObject.GetComponent<Reward.CreditsMissionReward>();
                        creditsMissionReward.Initialize((CreditsMissionReward)missionReward, ColorUtil.RarityColors.GetMissionRarityColor(selectedMission.MissionRarity));
                    }
                    else if (missionReward is ReputationMissionReward)
                    {
                        GameObject reputationMissionRewardObject = Instantiate(reputationMissionRewardPrefab, missionRewardParent);
                        Reward.ReputationMissionReward reputationMissionReward = reputationMissionRewardObject.GetComponent<Reward.ReputationMissionReward>();
                        reputationMissionReward.Initialize((ReputationMissionReward)missionReward, ColorUtil.RarityColors.GetMissionRarityColor(selectedMission.MissionRarity));
                    }
                    else if (missionReward is ItemMissionReward)
                    {
                        GameObject itemMissionRewardObject = Instantiate(itemMissionRewardPrefab, missionRewardParent);
                        Reward.ItemMissionReward itemMissionReward = itemMissionRewardObject.GetComponent<Reward.ItemMissionReward>();
                        itemMissionReward.Initialize((ItemMissionReward)missionReward, ColorUtil.RarityColors.GetMissionRarityColor(selectedMission.MissionRarity));
                    }
                } 
            }
        }

        private void RefreshMissionButtonCursor()
        {
            if (selectedMission == null)
            {
                if (missionButtonCursor != null)
                {
                    Destroy(missionButtonCursor.gameObject);
                }
            }
            else
            {
                if (missionButtonCursor == null)
                {
                    GameObject missionButtonCursorObject = Instantiate(missionButtonCursorPrefab);
                    missionButtonCursor = missionButtonCursorObject.GetComponent<MissionButtonCursor>();
                }
            }
        }

        private void SetPanelContentVisibility(bool visibility)
        {
            missionTitle.gameObject.SetActive(visibility);
            missionDescription.gameObject.SetActive(visibility);
            missionRequiredReputationLabel.gameObject.SetActive(visibility);
            missionRequiredReputationValue.gameObject.SetActive(visibility);
            missionTasks.gameObject.SetActive(visibility);
            missionTasksTitle.gameObject.SetActive(visibility);
            missionRewardTitle.gameObject.SetActive(visibility);
            missionAcceptButton.gameObject.SetActive(visibility);
            acceptedRarityMissionsLabel.gameObject.SetActive(visibility);
            acceptedRarityMissionsValue.gameObject.SetActive(visibility);
            acceptedTotalMissionsLabel.gameObject.SetActive(visibility);
            acceptedTotalMissionsValue.gameObject.SetActive(visibility);
        }

        private void SetRarityColor(Color rarityColor)
        {
            foreach (Image missionRarityBorderImage in missionRarityBorderImages)
            {
                missionRarityBorderImage.color = rarityColor;
            }
        }
    }
}