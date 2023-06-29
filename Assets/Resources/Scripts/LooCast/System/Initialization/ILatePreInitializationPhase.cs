namespace LooCast.System.Initialization
{
    public interface ILatePreInitializationPhase
    {
        #region Properties
        bool IsLatePreInitializing { get; }
        bool IsLatePreInitialized { get; }
        #endregion

        #region Methods
        void LatePreInitialize();
        #endregion
    }
}
