using System.Collections.Generic;
using UnityEngine.Events;

namespace LooCast.Mission.Task
{
    public abstract class MissionTask
    {
        private static Dictionary<int, MissionTask> missionTaskDictionary = new Dictionary<int, MissionTask>();
        private static int IDCounter = 0;
        public int ID { get; private set; }

        public UnityEvent OnComplete { get; private set; }
        public UnityEvent OnFinalize { get; private set; }

        public string Summary { get; private set; }
        public MissionTaskState MissionTaskState { get; private set; }

        protected MissionTask(string summary)
        {
            ID = IDCounter;
            IDCounter++;

            OnComplete = new UnityEvent();
            OnFinalize = new UnityEvent();

            Summary = summary;
            MissionTaskState = MissionTaskState.Incomplete;

            missionTaskDictionary.Add(ID, this);
        }

        ~MissionTask()
        {
            OnFinalize.Invoke();
            missionTaskDictionary.Remove(ID);
        }

        public virtual void Complete()
        {
            MissionTaskState = MissionTaskState.Complete;
            OnComplete.Invoke();
        }

        public override bool Equals(object obj)
        {
            MissionTask missionTask = (MissionTask)obj;
            if (missionTask != null && missionTask.ID == ID)
            {
                return true;
            }
            return false;
        }

        public override int GetHashCode()
        {
            return ID;
        }
    }
}