using System;
using System.Collections.Generic;
using System.Linq;
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
        public UnityEvent OnTaskStateChange { get; private set; }

        public string Summary { get; private set; }
        //public MissionTaskState MissionTaskState
        //{
        //    get
        //    {
        //        return missionTaskState;
        //    }
        //
        //    private set
        //    {
        //        missionTaskState = value;
        //        OnTaskStateChange.Invoke();
        //    }
        //}
        public List<MissionTask> SubTasks { get; private set; }

        //private MissionTaskState missionTaskState;

        protected MissionTask(string summary)
        {
            ID = IDCounter;
            IDCounter++;

            OnComplete = new UnityEvent();
            OnFinalize = new UnityEvent();
            OnTaskStateChange = new UnityEvent();

            Summary = summary;
            //MissionTaskState = MissionTaskState.Locked;
            SubTasks = new List<MissionTask>();

            missionTaskDictionary.Add(ID, this);
        }

        ~MissionTask()
        {
            OnFinalize.Invoke();
            missionTaskDictionary.Remove(ID);
        }

        public void AddSubTask(MissionTask subTask)
        {
            //MissionTaskState = MissionTaskState.Locked;
            SubTasks.Add(subTask);
            subTask.OnComplete.AddListener(() =>
            {
                //if (SubTasks.Where((task) => { return task.MissionTaskState == MissionTaskState.Incomplete; }).Count() == 0)
                //{
                //    Unlock();
                //}
            });
        }

        public virtual void Complete()
        {
            //if (MissionTaskState == MissionTaskState.Locked)
            //{
            //    throw new Exception("Mission can not be completed, when it is locked!");
            //}
            //MissionTaskState = MissionTaskState.Complete;
            //OnComplete.Invoke();
        }

        public void Unlock()
        {
            //MissionTaskState = MissionTaskState.Incomplete;
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