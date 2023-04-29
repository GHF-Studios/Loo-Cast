namespace LooCast.System
{
    public interface ILooCastObject
    {
        #region Properties

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
        void EarlyPreInitialize();

        void PreInitialize();

        void LatePreInitialize();

        void EarlyInitialize();

        void Initialize();

        void LateInitialize();

        void EarlyPostInitalize();

        void PostInitialize();

        void LatePostInitialize();
        #endregion

        #region Termination Phases
        void EarlyPreTerminate();

        void PreTerminate();

        void LatePreTerminate();

        void EarlyTerminate();

        void Terminate();

        void LateTerminate();

        void EarlyPostTerminate();

        void PostTerminate();

        void LatePostTerminate();
        #endregion
        
        #endregion
    }
}
