namespace LooCast.System.Termination
{
    public interface IFullPostTerminationPhase
    {
        #region Properties
        bool IsFullyPostTerminating { get; }
        bool IsFullyPostTermináted { get; }
        #endregion

        #region Methods
        void FullyPostTerminate();
        #endregion
    }
}
