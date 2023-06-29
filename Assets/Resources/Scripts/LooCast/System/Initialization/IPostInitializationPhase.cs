namespace LooCast.System.Initialization
{
    public interface IPostInitializationPhase
    {
        #region Properties
        bool IsPostInitializing { get; }
        bool IsPostInitialized { get; }
        #endregion

        #region Methods
        void PostInitialize();
        #endregion
    }
}
