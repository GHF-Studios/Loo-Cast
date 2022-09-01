using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Panel
{
    using LooCast.Mission;

    public class StationMissionPanel : Panel
    {
        public Mission SelectedMission
        {
            get
            {
                return selectedMission;
            }

            set
            {
                selectedMission = value;
            }
        }

        [SerializeField] private MissionProvider missionProvider;
        [SerializeField] private RectTransform missionButtonParent;
        [SerializeField] private GameObject missionButtonPrefab;
        [SerializeField] private Image[] missionRarityBorderImages;
        [SerializeField] private Text missionTitle;
        [SerializeField] private Text missionDescription;
        [SerializeField] private Text missionTasks;
        [SerializeField] private RectTransform missionRewardParent;
        [SerializeField] private GameObject missionRewardPrefab;
        
        private Mission selectedMission;

        public override void Refresh()
        {
            
        }
    }
}