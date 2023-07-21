namespace LooCast.System.Lifecycle.Initialization
{
    public interface IEarlyInitializationPhase
    {
        #region Properties
        bool IsEarlyInitializing { get; }
        bool IsEarlyInitialized { get; }
        #endregion

        #region Methods
        void OnEarlyInitialize();
        #endregion
    }
}
