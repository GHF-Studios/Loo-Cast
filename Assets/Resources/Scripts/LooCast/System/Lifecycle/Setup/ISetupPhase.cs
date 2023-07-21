namespace LooCast.System.Lifecycle.Setup
{
    public interface ISetupPhase
    {
        #region Properties
        bool IsSetupRunning { get; }
        bool IsSetupFinished { get; }
        #endregion

        #region Methods
        void OnSetup();
        #endregion
    }
}
