namespace LooCast.System
{
    public interface IEarlyPreInitializationPhase
    {
        #region Properties
        bool IsEarlyPreInitializing { get; }
        bool IsEarlyPreInitialized { get; }
        #endregion

        #region Methods
        void EarlyPreInitialize();
        #endregion
    }
}
