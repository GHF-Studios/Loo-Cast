namespace LooCast.System.Lifecycle.Initialization
{
    public interface IInitializationPhase
    {
        #region Properties
        bool IsInitializing { get; }
        bool IsInitialized { get; }
        #endregion

        #region Methods
        void OnInitialize();
        #endregion
    }
}
