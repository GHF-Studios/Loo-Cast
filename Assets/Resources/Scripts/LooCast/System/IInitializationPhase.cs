namespace LooCast.System
{
    public interface IInitializationPhase
    {
        #region Properties
        bool IsInitializing { get; }
        bool IsInitialized { get; }
        #endregion

        #region Methods
        void Initialize();
        #endregion
    }
}
