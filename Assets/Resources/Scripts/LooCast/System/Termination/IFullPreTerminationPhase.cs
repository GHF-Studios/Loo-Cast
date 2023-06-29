namespace LooCast.System.Termination
{
    public interface IFullPreTerminationPhase
    {
        #region Properties
        bool IsFullyPreTerminating { get; }
        bool IsFullyPreTerminated { get; }
        #endregion

        #region Methods
        void FullyPreTerminate();
        #endregion
    }
}
