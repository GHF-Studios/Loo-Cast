﻿namespace LooCast.System
{
    public interface IPreInitializationPhase
    {
        #region Properties
        bool IsPreInitializing { get; }
        bool IsPreInitialized { get; }
        #endregion

        #region Methods
        void PreInitialize();
        #endregion
    }
}
