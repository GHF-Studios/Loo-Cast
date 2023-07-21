namespace LooCast.System.Lifecycle.Setup
{
    public interface IPostSetupPhase
    {
        #region Properties
        bool IsPostSetupRunning { get; }
        bool IsPostSetupFinished { get; }
        #endregion

        #region Methods
        void OnPostSetup();
        #endregion
    }
}
