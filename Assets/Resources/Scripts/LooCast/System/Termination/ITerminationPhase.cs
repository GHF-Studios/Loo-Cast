namespace LooCast.System.Termination
{
    public interface ITerminationPhase
    {
        #region Properties
        bool IsTerminating { get; }
        bool IsTerminated { get; }
        #endregion

        #region Methods
        void Terminate();
        #endregion
    }
}
