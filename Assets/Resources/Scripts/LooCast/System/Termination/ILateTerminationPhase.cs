namespace LooCast.System.Termination
{
    public interface ILateTerminationPhase
    {
        #region Properties
        bool IsLateTerminating { get; }
        bool IsLateTerminated { get; }
        #endregion

        #region Methods
        void LateTerminate();
        #endregion
    }
}
