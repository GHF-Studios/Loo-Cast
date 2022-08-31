using UnityEngine.Events;

namespace LooCast.Mission
{
    public class MissionTask
    {
        public abstract UnityEvent OnTaskComplete { get; }
    }
}