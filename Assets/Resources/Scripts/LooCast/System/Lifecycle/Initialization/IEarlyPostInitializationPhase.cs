namespace LooCast.System.Lifecycle.Initialization
{
    public interface IEarlyPostInitializationPhase
    {
        #region Properties
        bool IsEarlyPostInitializing { get; }
        bool IsEarlyPostInitialized { get; }
        #endregion

        #region Methods
        void OnEarlyPostInitialize();
        #endregion
    }
}
