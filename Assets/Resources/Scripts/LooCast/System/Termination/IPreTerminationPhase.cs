namespace LooCast.System.Termination
{
    public interface IPreTerminationPhase
    {
        #region Properties
        bool IsPreTerminating { get; }
        bool IsPreTerminated { get; }
        #endregion

        #region Methods
        void PreTerminate();
        #endregion
    }
}
