namespace LooCast.Mission.Task
{
    public class LockedMissionTaskLockState : IMissionTaskLockState
    {
        private MissionTask missionTask;

        public LockedMissionTaskLockState(MissionTask missionTask)
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