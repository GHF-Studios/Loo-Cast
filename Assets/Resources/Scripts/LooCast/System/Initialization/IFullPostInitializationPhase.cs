namespace LooCast.System.Initialization
{
    public interface IFullPostInitializationPhase
    {
        #region Properties
        bool IsFullyPostInitializing { get; }
        bool IsFullyPostInitialized { get; }
        #endregion

        #region Methods
        void FullyPostInitialize();
        #endregion
    }
}
