namespace LooCast.System.Lifecycle.Initialization
{
    public interface IPreInitializationPhase
    {
        #region Properties
        bool IsPreInitializing { get; }
        bool IsPreInitialized { get; }
        #endregion

        #region Methods
        void OnPreInitialize();
        #endregion
    }
}
