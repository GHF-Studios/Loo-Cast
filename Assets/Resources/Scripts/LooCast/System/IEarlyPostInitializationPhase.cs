namespace LooCast.System
{
    public interface IEarlyPostInitializationPhase
    {
        #region Properties
        bool IsEarlyPostInitializing { get; }
        bool IsEarlyPostInitialized { get; }
        #endregion

        #region Methods
        void EarlyPostInitialize();
        #endregion
    }
}
