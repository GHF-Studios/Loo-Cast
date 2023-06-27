namespace LooCast.System
{
    public interface IFullInitializationPhase : IEarlyInitializationPhase, IInitializationPhase, ILateInitializationPhase
    {
        #region Properties
        bool IsFullyInitializing { get; }
        bool IsFullyInitialized { get; }
        #endregion
    }
}
