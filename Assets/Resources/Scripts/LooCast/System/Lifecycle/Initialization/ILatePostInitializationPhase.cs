namespace LooCast.System.Lifecycle.Initialization
{
    public interface ILatePostInitializationPhase
    {
        #region Properties
        bool IsLatePostInitializing { get; }
        bool IsLatePostInitialized { get; }
        #endregion

        #region Methods
        void OnLatePostInitialize();
        #endregion
    }
}
