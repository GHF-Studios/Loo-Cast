namespace LooCast.System.Termination
{
    public interface IEarlyPreTerminationPhase
    {
        #region Properties
        bool IsEarlyPreTerminating { get; }
        bool IsEarlyPreTerminated { get; }
        #endregion

        #region Methods
        void EarlyPreTerminate();
        #endregion
    }
}
