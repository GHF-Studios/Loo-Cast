namespace LooCast.System.Lifecycle.Initialization
{
    public interface IPostInitializationPhase
    {
        #region Properties
        bool IsPostInitializing { get; }
        bool IsPostInitialized { get; }
        #endregion

        #region Methods
        void OnPostInitialize();
        #endregion
    }
}
