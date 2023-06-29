namespace LooCast.System.Termination
{
    public interface ILatePostTerminationPhase
    {
        #region Properties
        bool IsLatePostTerminating { get; }
        bool IsLatePostTerminated { get; }
        #endregion

        #region Methods
        void LatePostTerminate();
        #endregion
    }
}
