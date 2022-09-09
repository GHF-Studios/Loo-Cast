using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.UI.Task
{
    public class MissionTaskContainer : MonoBehaviour
    {
        [SerializeField] private GameObject uiTaskPrefab;

        public void ClearTasks()
        {
            foreach (Transform task in transform)
            {
                Destroy(task.gameObject);
            }
        }

        public void AddTask(Mission.Task.MissionTask task)
        {
            GameObject uiTaskObject = Instantiate(uiTaskPrefab, transform);
            MissionTask uiTask = uiTaskObject.GetComponent<MissionTask>();
            uiTask.Initialize(task);
            foreach (Mission.Task.MissionTask subTask in task.SubTasks)
            {
                uiTask.AddSubTask(subTask);
            }
        }
    }
}
