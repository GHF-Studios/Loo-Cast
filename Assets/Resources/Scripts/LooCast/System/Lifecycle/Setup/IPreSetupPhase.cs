namespace LooCast.System.Lifecycle.Setup
{
    public interface IPreSetupPhase
    {
        #region Propertirs
        bool IsPreSetupRunning { get; }
        bool IsPreSetupFinished { get; }
        #endregion

        #region Methods
        void OnPreSetup();
        #endregion
    }
}
