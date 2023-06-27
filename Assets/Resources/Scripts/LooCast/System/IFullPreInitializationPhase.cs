namespace LooCast.System
{
    public interface IFullPreInitializationPhase : IEarlyPreInitializationPhase, IPreInitializationPhase, ILatePreInitializationPhase
    {
        #region Properties
        bool IsFullyPreInitializing { get; }
        bool IsFullyPreInitialized { get; }
        #endregion
    }
}
