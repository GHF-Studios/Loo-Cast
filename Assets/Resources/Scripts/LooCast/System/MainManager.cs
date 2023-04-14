﻿using System;

namespace LooCast.System
{
    using global::LooCast.System.Hierarchies;
    using global::LooCast.System.MetaData;
    using global::LooCast.System.Registries;
    using UnityEngine;

    public sealed class MainManager : Manager<MainManager, MainManagerMetaData>
    {
        #region Properties
        public ICoreModuleManager[] CoreModuleManagers { get; private set; }
        public MainRegistry MainRegistry { get; private set; }
        public MainHierarchy MainHierarchy { get; private set; }
        #endregion

        #region Callbacks

        #region Initialization Phases
        private void OnEarlyPreInitialize()
        {
            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.EarlyPreInitialize();
            }
        }

        private void OnPreInitialize()
        {
            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.PreInitialize();
            }
        }

        private void OnLatePreInitialize()
        {
            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.LatePreInitialize();
            }
        }

        private void OnEarlyInitialize()
        {
            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.EarlyInitialize();
            }
        }

        private void OnInitialize()
        {
            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.Initialize();
            }
        }

        private void OnLateInitialize()
        {
            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.LateInitialize();
            }
        }

        private void OnEarlyPostInitialize()
        {
            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.EarlyPostInitalize();
            }
        }

        private void OnPostInitialize()
        {
            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.PostInitialize();
            }
        }

        private void OnLatePostInitialize()
        {
            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.LatePostInitialize();
            }
        }
        #endregion

        #region Termination Phases
        private void OnEarlyPreTerminate()
        {
            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.EarlyPreTerminate();
            }
        }

        private void OnPreTerminate()
        {
            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.PreTerminate();
            }
        }

        private void OnLatePreTerminate()
        {
            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.LatePreTerminate();
            }
        }

        private void OnEarlyTerminate()
        {
            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.EarlyTerminate();
            }
        }

        private void OnTerminate()
        {
            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.Terminate();
            }
        }

        private void OnLateTerminate()
        {
            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.LateTerminate();
            }
        }

        private void OnEarlyPostTerminate()
        {
            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.EarlyPostTerminate();
            }
        }

        private void OnPostTerminate()
        {
            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.PostTerminate();
            }
        }

        private void OnLatePostTerminate()
        {
            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.LatePostTerminate();
            }
        }
        #endregion

        #endregion
        
        #region Unity Callbacks

        #region Initialization
        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
        private static void PreAwake()
        {
            // Pre-Initialize the Main Manager
            Instance.EarlyPreInitialize();

            // Pre-Initialize all the core module managers
            foreach (ICoreModuleManager coreModuleManager in Instance.CoreModuleManagers)
            {
                coreModuleManager.EarlyPreInitialize();
            }
        }

        private void Awake()
        {
            EarlyInitialize();

            // Pre-Initialize all the core module managers
            foreach (ICoreModuleManager coreModuleManager in Instance.CoreModuleManagers)
            {
                coreModuleManager.EarlyInitialize();
            }
        }

        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.AfterSceneLoad)]
        private static void PostAwake()
        {
            Instance.EarlyPostInitalize();

            // Pre-Initialize all the core module managers
            foreach (ICoreModuleManager coreModuleManager in Instance.CoreModuleManagers)
            {
                coreModuleManager.EarlyPostInitalize();
            }
        }
        #endregion

        #region Termination
        private void OnDisable()
        {
            if (!IsEarlyTerminating && !IsTerminating && !IsPostTerminating && !IsEarlyTerminated && !IsTerminated && !IsPostTerminated)
            {
                EarlyPreTerminate();
            }
        }

        private void OnApplicationQuit()
        {
            if (!IsEarlyTerminating && !IsTerminating && !IsPostTerminating && !IsEarlyTerminated && !IsTerminated && !IsPostTerminated)
            {
                EarlyPreTerminate();
            }
        }
        #endregion

        #endregion

        #region Methods
        /// <summary>
        /// Returns the core module managers in the order they should be initialized.
        /// </summary>
        private ICoreModuleManager[] GetCoreModuleManagers()
        {
            return new ICoreModuleManager[]
            {
                // TODO: Read the mod folder for valid core module managers and load them
                global::LooCast.Core.CoreManager.Instance
            };
        }
        #endregion

        #region Overrides
        protected override void PreConstruct()
        {
            base.PreConstruct();

            RegisterEarlyPreInitializationAction(OnEarlyPreInitialize);
            RegisterPreInitializationAction(OnPreInitialize);
            RegisterLatePreInitializationAction(OnLatePreInitialize);
            RegisterEarlyInitializationAction(OnEarlyInitialize);
            RegisterInitializationAction(OnInitialize);
            RegisterLateInitializationAction(OnLateInitialize);
            RegisterEarlyPostInitializationAction(OnEarlyPostInitialize);
            RegisterPostInitializationAction(OnPostInitialize);
            RegisterLatePostInitializationAction(OnLatePostInitialize);

            RegisterEarlyPreTerminationAction(OnEarlyPreTerminate);
            RegisterPreTerminationAction(OnPreTerminate);
            RegisterLatePreTerminationAction(OnLatePreTerminate);
            RegisterEarlyTerminationAction(OnEarlyTerminate);
            RegisterTerminationAction(OnTerminate);
            RegisterLateTerminationAction(OnLateTerminate);
            RegisterEarlyPostTerminationAction(OnEarlyPostTerminate);
            RegisterPostTerminationAction(OnPostTerminate);
            RegisterLatePostTerminationAction(OnLatePostTerminate);

            CoreModuleManagers = GetCoreModuleManagers();
            MainRegistry = new MainRegistry();
            MainHierarchy = new MainHierarchy();
            
            // TODO: 1. Register all system registries
            // TODO: 2. Register all system hierarchies
            // TODO: 3. Register every Namespace & Type that's part of the core system in the respective registries and hierarchies
        }

        protected override void Construct()
        {
            base.Construct();
        }

        protected override void PostConstruct()
        {
            base.PostConstruct();
        }
        #endregion
    }
}
