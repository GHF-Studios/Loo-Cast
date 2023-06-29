namespace LooCast.System.Initialization
{
    public interface IFullInitializationPhase
    {
        #region Properties
        bool IsFullyInitializing { get; }
        bool IsFullyInitialized { get; }
        #endregion

        #region Methods
        void FullyInitialize();
        #endregion
    }
}
