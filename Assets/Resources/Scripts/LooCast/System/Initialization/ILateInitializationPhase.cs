namespace LooCast.System.Initialization
{
    public interface ILateInitializationPhase
    {
        #region Properties
        bool IsLateInitializing { get; }
        bool IsLateInitialized { get; }
        #endregion

        #region Methods
        void LateInitialize();
        #endregion
    }
}
