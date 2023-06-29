namespace LooCast.System.Termination
{
    public interface IEarlyPostTerminationPhase
    {
        #region Properties
        bool IsEarlyPostTerminating { get; }
        bool IsEarlyPostTerminated { get; }
        #endregion

        #region Methods
        void EarlyPostTerminate();
        #endregion
    }
}
