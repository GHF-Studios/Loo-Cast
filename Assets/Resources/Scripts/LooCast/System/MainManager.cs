using UnityEngine;
using System;

namespace LooCast.System
{
    public sealed class MainManager : Manager
    {
        #region Static Properties
        public static MainManager Instance
        {
            get
            {
                if (Instance == null)
                {
                    instance = new MainManager();
                }
                return instance;
            }
        }
        #endregion
        
        #region Static Fields
        private static MainManager instance;
        #endregion
        
        #region Properties
        public IFolder RootFolder { get; private set; }
        public ICoreModuleManager[] CoreModuleManagers { get; private set; }
        #endregion

        #region Constructors
        private MainManager() : base("MainManager", null, ManagerMonoBehaviour.CreateManagerObject("MainManager"))
        {
            RootFolder = new Folder();
            CoreModuleManagers = new ICoreModuleManager[]
            {
                // TODO: Read the mod folder for valid core module managers and load them.
                // LooCast.Core.CoreManager.Instance,
                // ThermalDynamics.Core.CoreManager.Instance,
                // ThermalExpansion.Core.CoreManager.Instance,
                // CrazySexMod.Core.CoreManager.Instance
            };

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
        }
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
                coreModuleManager.EarlyPostInitialize();
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
            Instance.EarlyPostInitialize();

            // Pre-Initialize all the core module managers
            foreach (ICoreModuleManager coreModuleManager in Instance.CoreModuleManagers)
            {
                coreModuleManager.EarlyPostInitialize();
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
    }
}
