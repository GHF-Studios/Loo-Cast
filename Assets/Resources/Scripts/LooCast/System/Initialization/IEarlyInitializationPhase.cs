namespace LooCast.System.Initialization
{
    public interface IEarlyInitializationPhase
    {
        #region Properties
        bool IsEarlyInitializing { get; }
        bool IsEarlyInitialized { get; }
        #endregion

        #region Methods
        void EarlyInitialize();
        #endregion
    }
}
