namespace LooCast.System.Termination
{
    public interface IFullTerminationPhase
    {
        #region Properties
        bool IsFullyTerminating { get; }
        bool IsFullyTerminated { get; }
        #endregion

        #region Methods
        void FullyTerminate();
        #endregion
    }
}
