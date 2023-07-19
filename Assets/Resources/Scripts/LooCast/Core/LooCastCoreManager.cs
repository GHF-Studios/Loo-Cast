﻿namespace LooCast.Core
{
    using LooCast.System;
    using LooCast.System.Numerics;
    using LooCast.Universe;
    
    public sealed class LooCastCoreManager : CoreModuleManager
    {
        #region Static Properties
        public static LooCastCoreManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new LooCastCoreManager();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static LooCastCoreManager instance;
        #endregion

        #region Properties
        public Universe Universe { get; private set; }
        public UniverseObserver UniverseObserver { get; private set; }
        #endregion

        #region Constructors
        private LooCastCoreManager() : base("LooCastCoreManager")
        {
            RegisterInitializationAction(OnInitialization);
        }
        #endregion

        #region Callbacks
        private void OnInitialization()
        {
            Universe = new Universe(64);
            UniverseObserver = new UniverseObserver(256);
        }
        #endregion
    }
}
