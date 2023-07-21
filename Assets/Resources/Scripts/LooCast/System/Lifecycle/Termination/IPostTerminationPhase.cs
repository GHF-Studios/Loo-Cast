namespace LooCast.System.Lifecycle.Termination
{
    public interface IPostTerminationPhase
    {
        #region Properties
        bool IsPostTerminating { get; }
        bool IsPostTerminated { get; }
        #endregion

        #region Methods
        void OnPostTerminate();
        #endregion
    }
}
