namespace LooCast.System
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
