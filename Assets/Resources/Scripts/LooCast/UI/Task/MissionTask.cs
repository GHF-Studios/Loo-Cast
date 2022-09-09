using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

namespace LooCast.UI.Task
{
    public class MissionTask : MonoBehaviour
    {
        [SerializeField] private Text summaryText;
        [SerializeField] private MissionTaskContainer subTasks;

        public void Initialize(Mission.Task.MissionTask task)
        {
            summaryText.text = task.Summary;
        }

        public void AddSubTask(Mission.Task.MissionTask subTask)
        {
            subTasks.AddTask(subTask);
        }
    }
}
