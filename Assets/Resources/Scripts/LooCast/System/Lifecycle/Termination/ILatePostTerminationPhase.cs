namespace LooCast.System.Lifecycle.Termination
{
    public interface ILatePostTerminationPhase
    {
        #region Properties
        bool IsLatePostTerminating { get; }
        bool IsLatePostTerminated { get; }
        #endregion

        #region Methods
        void OnLatePostTerminate();
        #endregion
    }
}
