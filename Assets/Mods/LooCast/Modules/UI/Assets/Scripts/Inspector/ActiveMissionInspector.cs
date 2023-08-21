using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Inspector
{
    using LooCast.UI.Task;
    using LooCast.Mission;
    
    public class ActiveMissionInspector : MonoBehaviour
    {
        public Mission ActiveMission
        {
            get
            {
                return activeMission;
            }

            set
            {
                activeMission = value;
                if (activeMission == null)
                {
                    gameObject.SetActive(false);
                    tasks.ClearTasks();
                }
                else
                {
                    gameObject.SetActive(true);
                    tasks.ClearTasks();
                    tasks.AddTask(activeMission.RootMissionTask);
                }
            }
        }

        [SerializeField] private MissionTaskContainer tasks;

        private Mission activeMission;

        private void Start()
        {
            ActiveMission = null;
            MissionManager.Instance.OnActiveMissionChange.AddListener((activeMission) => { ActiveMission = activeMission; });
        }
    }
}