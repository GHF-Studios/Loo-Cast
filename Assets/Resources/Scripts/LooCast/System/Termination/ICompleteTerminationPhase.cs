namespace LooCast.System.Termination
{
    public interface ICompleteTerminationPhase
    {
        #region Properties
        bool IsCompletelyPreTerminating { get; }
        bool IsCompletelyPreTerminated { get; }
        #endregion

        #region Methods
        void CompletelyTerminate();
        #endregion
    }
}
