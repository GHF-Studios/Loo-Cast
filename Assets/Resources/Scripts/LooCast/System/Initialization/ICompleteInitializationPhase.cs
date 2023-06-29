namespace LooCast.System.Initialization
{
    public interface ICompleteInitializationPhase
    {
        #region Properties
        bool IsCompletelyInitializing { get; }
        bool IsCompletelyInitialized { get; }
        #endregion

        #region Methods
        void CompletelyInitialize();
        #endregion
    }
}
