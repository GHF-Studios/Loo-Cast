namespace LooCast.System
{
    public interface ICompleteInitializationPhase : IFullPreInitializationPhase, IFullInitializationPhase, IFullPostInitializationPhase
    {
        #region Properties
        bool IsCompletelyInitializing { get; }
        bool IsCompletelyInitialized { get; }
        #endregion
    }
}
