namespace LooCast.Mission.Task
{
    public interface IMissionTaskLockState
    {
        IMissionTaskLockState Validate();
        void Enter();
        void Update();
        void Exit();
    }
}