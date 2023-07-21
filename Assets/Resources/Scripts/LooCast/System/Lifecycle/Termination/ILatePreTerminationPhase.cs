namespace LooCast.System.Lifecycle.Termination
{
    public interface ILatePreTerminationPhase
    {
        #region Properties
        bool IsLatePreTerminating { get; }
        bool IsLatePreTerminated { get; }
        #endregion

        #region Methods
        void OnLatePreTerminate();
        #endregion
    }
}
