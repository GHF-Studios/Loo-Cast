namespace LooCast.System.Lifecycle.Termination
{
    public interface ITerminationPhase
    {
        #region Properties
        bool IsTerminating { get; }
        bool IsTerminated { get; }
        #endregion

        #region Methods
        void OnTerminate();
        #endregion
    }
}
