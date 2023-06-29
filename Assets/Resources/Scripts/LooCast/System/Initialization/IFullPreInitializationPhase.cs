namespace LooCast.System.Initialization
{
    public interface IFullPreInitializationPhase
    {
        #region Properties
        bool IsFullyPreInitializing { get; }
        bool IsFullyPreInitialized { get; }
        #endregion

        #region Methods
        void FullyPreInitialize();
        #endregion
    }
}
