namespace LooCast.System.Lifecycle.Termination
{
    public interface IEarlyPreTerminationPhase
    {
        #region Properties
        bool IsEarlyPreTerminating { get; }
        bool IsEarlyPreTerminated { get; }
        #endregion

        #region Methods
        void OnEarlyPreTerminate();
        #endregion
    }
}
