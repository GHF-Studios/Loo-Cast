using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Task
{
    public class MissionTask : MonoBehaviour
    {
        [SerializeField] private Text summaryText;
        [SerializeField] private GameObject subTasksParent;
        [SerializeField] private MissionTaskContainer subTaskContainer;

        public void Initialize(Mission.Task.MissionTask task)
        {
            //summaryText.text = task.Summary;
            //subTasksParent.SetActive(false);
            //task.OnTaskStateChange.AddListener(() => { RefreshTaskState(task.MissionTaskState); });
            //RefreshTaskState(task.MissionTaskState);
        }

        public void AddSubTask(Mission.Task.MissionTask subTask)
        {
            //if (!subTasksParent.activeSelf)
            //{
            //    subTasksParent.SetActive(true);
            //}
            //subTaskContainer.AddTask(subTask);
        }

        //private void RefreshTaskState(Mission.Task.MissionTaskState taskState)
        //{
        //    summaryText.color = GetTaskStateColor(taskState);
        //}

        //private Color GetTaskStateColor(Mission.Task.MissionTaskState taskState)
        //{
        //    switch (taskState)
        //    {
        //        case Mission.Task.MissionTaskState.Incomplete:
        //            return Color.yellow;
        //        case Mission.Task.MissionTaskState.Complete:
        //            return Color.green;
        //        case Mission.Task.MissionTaskState.Locked:
        //            return Color.red;
        //        default:
        //            throw new ArgumentException("Invalid Task State!");
        //    }
        //}
    }
}
