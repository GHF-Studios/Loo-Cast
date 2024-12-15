namespace LooCast.Mission.Task
{
    public class UnlockedMissionTaskLockState : IMissionTaskLockState
    {
        private MissionTask missionTask;

        public UnlockedMissionTaskLockState(MissionTask missionTask)
        {
            this.missionTask = missionTask;
        }

        public IMissionTaskLockState Validate()
        {
            return this;
        }

        public void Enter()
        {

        }

        public void Update()
        {

        }

        public void Exit()
        {

        }
    }
}