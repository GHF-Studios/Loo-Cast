using System;

namespace LooCast.System
{
    public interface IManager
    {
        #region Properties
        public ManagerObject ManagerObject { get; }
#nullable enable
        public IManager? ParentManager { get; }
#nullable disable

        #region Initialization Phase Flags
        public bool IsEarlyPreInitializing { get; }
        public bool IsPreInitializing { get; }
        public bool IsLatePreInitializing { get; }
        public bool IsEarlyPreInitialized { get; }
        public bool IsPreInitialized { get; }
        public bool IsLatePreInitialized { get; }

        public bool IsEarlyInitializing { get; }
        public bool IsInitializing { get; }
        public bool IsLateInitializing { get; }
        public bool IsEarlyInitialized { get; }
        public bool IsInitialized { get; }
        public bool IsLateInitialized { get; }

        public bool IsEarlyPostInitializing { get; }
        public bool IsPostInitializing { get; }
        public bool IsLatePostInitializing { get; }
        public bool IsEarlyPostInitialized { get; }
        public bool IsPostInitialized { get; }
        public bool IsLatePostInitialized { get; }

        public bool IsFullyPreInitialized
        {
            get
            {
                return IsEarlyPreInitialized && IsPreInitialized && IsLatePreInitialized;
            }
        }
        public bool IsFullyInitialized
        {
            get
            {
                return IsEarlyInitialized && IsInitialized && IsLateInitialized;
            }
        }
        public bool IsFullyPostInitialized
        {
            get
            {
                return IsEarlyPostInitialized && IsPostInitialized && IsLatePostInitialized;
            }
        }
        public bool IsCompletelyInitialized
        {
            get
            {
                return IsFullyPreInitialized && IsFullyInitialized && IsPostInitialized;
            }
        }
        #endregion

        #region Termination Phase Flags
        public bool IsEarlyPreTerminating { get; }
        public bool IsPreTerminating { get; }
        public bool IsLatePreTerminating { get; }
        public bool IsEarlyPreTerminated { get; }
        public bool IsPreTerminated { get; }
        public bool IsLatePreTerminated { get; }

        public bool IsEarlyTerminating { get; }
        public bool IsTerminating { get; }
        public bool IsLateTerminating { get; }
        public bool IsEarlyTerminated { get; }
        public bool IsTerminated { get; }
        public bool IsLateTerminated { get; }

        public bool IsEarlyPostTerminating { get; }
        public bool IsPostTerminating { get; }
        public bool IsLatePostTerminating { get; }
        public bool IsEarlyPostTerminated { get; }
        public bool IsPostTerminated { get; }
        public bool IsLatePostTerminated { get; }

        public bool IsFullyPreTerminated
        {
            get
            {
                return IsEarlyPreTerminated && IsPreTerminated && IsLatePreTerminated;
            }
        }
        public bool IsFullyTerminated
        {
            get
            {
                return IsEarlyTerminated && IsTerminated && IsLateTerminated;
            }
        }
        public bool IsFullyPostTerminated
        {
            get
            {
                return IsEarlyPostTerminated && IsPostTerminated && IsLatePostTerminated;
            }
        }
        public bool IsCompletelyTerminated
        {
            get
            {
                return IsFullyPreTerminated && IsFullyTerminated && IsPostTerminated;
            }
        }
        #endregion

        #endregion

        #region Methods

        #region Initialization Phases
        public void EarlyPreInitialize();

        public void PreInitialize();

        public void LatePreInitialize();

        public void EarlyInitialize();

        public void Initialize();

        public void LateInitialize();

        public void EarlyPostInitalize();

        public void PostInitialize();

        public void LatePostInitialize();
        #endregion

        #region Termination Phases
        public void EarlyPreTerminate();

        public void PreTerminate();

        public void LatePreTerminate();

        public void EarlyTerminate();

        public void Terminate();

        public void LateTerminate();

        public void EarlyPostTerminate();

        public void PostTerminate();

        public void LatePostTerminate();
        #endregion

        #region Initialization Action Registration
        public void RegisterEarlyPreInitializationAction(Action action);

        public void RegisterPreInitializationAction(Action action);

        public void RegisterLatePreInitializationAction(Action action);

        public void RegisterEarlyInitializationAction(Action action);

        public void RegisterInitializationAction(Action action);

        public void RegisterLateInitializationAction(Action action);

        public void RegisterEarlyPostInitializationAction(Action action);

        public void RegisterPostInitializationAction(Action action);

        public void RegisterLatePostInitializationAction(Action action);
        #endregion

        #region Termination Action Registration
        public void RegisterEarlyPreTerminationAction(Action action);

        public void RegisterPreTerminationAction(Action action);

        public void RegisterLatePreTerminationAction(Action action);

        public void RegisterEarlyTerminationAction(Action action);

        public void RegisterTerminationAction(Action action);

        public void RegisterLateTerminationAction(Action action);

        public void RegisterEarlyPostTerminationAction(Action action);

        public void RegisterPostTerminationAction(Action action);

        public void RegisterLatePostTerminationAction(Action action);
        #endregion
        
        #endregion
    }
}
