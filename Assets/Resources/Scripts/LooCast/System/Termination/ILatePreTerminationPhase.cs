namespace LooCast.System.Termination
{
    public interface ILatePreTerminationPhase
    {
        #region Properties
        bool IsLatePreTerminating { get; }
        bool IsLatePreTerminated { get; }
        #endregion

        #region Methods
        void LatePreTerminate();
        #endregion
    }
}
