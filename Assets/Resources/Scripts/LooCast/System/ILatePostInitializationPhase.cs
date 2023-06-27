namespace LooCast.System
{
    public interface ILatePostInitializationPhase
    {
        #region Properties
        bool IsLatePostInitializing { get; }
        bool IsLatePostInitialized { get; }
        #endregion

        #region Methods
        void LatePostInitialize();
        #endregion
    }
}
