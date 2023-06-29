namespace LooCast.System.Termination
{
    public interface IPostTerminationPhase
    {
        #region Properties
        bool IsPostTerminating { get; }
        bool IsPostTerminated { get; }
        #endregion

        #region Methods
        void PostTerminate();
        #endregion
    }
}
