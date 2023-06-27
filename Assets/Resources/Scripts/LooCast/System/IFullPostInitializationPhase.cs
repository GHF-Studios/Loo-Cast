namespace LooCast.System
{
    public interface IFullPostInitializationPhase : IEarlyPostInitializationPhase, IPostInitializationPhase, ILatePostInitializationPhase
    {
        #region Properties
        bool IsFullyPostInitializing { get; }
        bool IsFullyPostInitialized { get; }
        #endregion
    }
}
