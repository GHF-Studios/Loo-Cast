namespace LooCast.System.Lifecycle.Termination
{
    public interface IPreTerminationPhase
    {
        #region Properties
        bool IsPreTerminating { get; }
        bool IsPreTerminated { get; }
        #endregion

        #region Methods
        void OnPreTerminate();
        #endregion
    }
}
