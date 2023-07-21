namespace LooCast.System.Lifecycle.Termination
{
    public interface IEarlyTerminationPhase
    {
        #region Properties
        bool IsEarlyTerminating { get; }
        bool IsEarlyTerminated { get; }
        #endregion

        #region Methods
        void OnEarlyTerminate();
        #endregion
    }
}
