using System;

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
        private static void OnEarlyPreInitialize()
        {
            Instance.EarlyPreInitialize();
        }

        private void Awake()
        {
            EarlyInitialize();
        }

        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.AfterSceneLoad)]
        private static void OnEarlyPostInitialize()
        {
            Instance.EarlyPostInitialize();
        }
        #endregion

        #region Termination
        private void OnDisable()
        {
            EarlyPreTerminate();
        }

        private void OnDestroy()
        {
            EarlyPreTerminate();
        }

        private void OnApplicationQuit()
        {
            EarlyPreTerminate();
        }
        #endregion

        #endregion

        #region Methods
        // Add methods for managing foundational types of Loo Cast Objects

        #region Initialization Phases
        private void EarlyPreInitialize()
        {
            internalManagers = new InternalManager[]
            {
                MainManager.Instance,
                RegistryManager.Instance,
                NamespaceManager.Instance,
                TypeManager.Instance,
                SystemObjectManager.Instance,
                GameObjectManager.Instance,
                ComponentManager.Instance
            };

            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;

            NamespaceManager namespaceManager = NamespaceManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            SystemObjectManager systemObjectManager = SystemObjectManager.Instance;
            GameObjectManager gameObjectManager = GameObjectManager.Instance;

            #region Internal Managers Setup

            #region Pre-Initialization
            Debug.Log($"[LooCast] Pre-Initializing internal module manager instances.");
            foreach (InternalManager internalManager in internalManagers)
            {
                internalManager.PreInitializeInstance();
            }
            Debug.Log($"[LooCast] Pre-Initialized internal module manager instances.");
            #endregion

            #region Initialization
            Debug.Log($"[LooCast] Initializing internal module manager instances.");
            foreach (InternalManager internalManager in internalManagers)
            {
                internalManager.InitializeInstance();
            }
            Debug.Log($"[LooCast] Initialized internal module manager instances.");
            #endregion

            #region Post-Initialization
            Debug.Log($"[LooCast] Post-Initializing internal module manager instances.");
            foreach (InternalManager internalManager in internalManagers)
            {
                internalManager.PostInitializeInstance();
            }
            Debug.Log($"[LooCast] Post-Initialized internal module manager instances.");
            #endregion

            #endregion

            IsEarlyPreInitializing = true;
            Debug.Log($"[LooCast] Starting Early Pre-Initialization in Scene '{activeSceneName}'.");

            #region Early Pre-Initialization

            #region Internal Managers
            foreach (InternalManager internalManager in internalManagers)
            {
                internalManager.EarlyPreInitialize();
            }
            #endregion

            #endregion

            IsEarlyPreInitializing = false;
            IsEarlyPreInitialized = true;
            Debug.Log($"[LooCast] Finished Early Pre-Initialization in Scene '{activeSceneName}'.");

            PreInitialize();
        }

        private void PreInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsPreInitializing = true;
            Debug.Log($"[LooCast] Starting Pre-Initialization in Scene '{activeSceneName}'.");

            #region Pre-Initialization

            #region Internal Managers
            foreach (InternalManager internalManager in internalManagers)
            {
                internalManager.PreInitialize();
            }
            #endregion

            #endregion

            IsPreInitializing = false;
            IsPreInitialized = true;
            Debug.Log($"[LooCast] Finished Pre-Initialization in Scene '{activeSceneName}'.");

            LatePreInitialize();
        }

        private void LatePreInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLatePreInitializing = true;
            Debug.Log($"[LooCast] Starting Pre-Initialization in Scene '{activeSceneName}'.");

            #region Late Pre-Initialization

            #region Internal Managers
            foreach (InternalManager internalManager in internalManagers)
            {
                internalManager.LatePreInitialize();
            }
            #endregion

            #endregion

            IsPreInitializing = false;
            IsPreInitialized = true;
            Debug.Log($"[LooCast] Finished Pre-Initialization in Scene '{activeSceneName}'.");

            _ = Instance;
        }

        private void EarlyInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsEarlyInitializing = true;
            Debug.Log($"[LooCast] Starting Early Pre-Initialization in Scene '{activeSceneName}'.");

            #region Early Initialization

            #region Internal Managers
            foreach (InternalManager internalManager in internalManagers)
            {
                internalManager.EarlyInitialize();
            }
            #endregion

            #endregion

            IsEarlyInitializing = false;
            IsEarlyInitialized = true;
            Debug.Log($"[LooCast] Finished Early Pre-Initialization in Scene '{activeSceneName}'.");

            Initialize();
        }

        private void Initialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsInitializing = true;
            Debug.Log($"[LooCast] Starting Initialization in Scene '{activeSceneName}'.");

            #region Initialization

            #region Internal Managers
            foreach (InternalManager internalManager in internalManagers)
            {
                internalManager.Initialize();
            }
            #endregion

            // TODO: SteamManager, Utilities and Scene should not be initialized here!
            #region SteamManager
            _ = SteamworksManager.Initialized;
            #endregion

            #region Utilities
            Universe.Universe.DensityMapGenerationUtil.InitializeInstance();
            #endregion

            #region Scene
            switch (activeSceneName)
            {
                case "MainMenu":
                    break;
                case "Game":
                    SceneManager.Instance.AddPostSceneLoadAction(() =>
                    {
                        GameManager gameManager = FindObjectOfType<GameManager>();
                        if (gameManager.Games.Contains("New Game"))
                        {
                            gameManager.InitializeGame(gameManager.Games.GetGame("New Game"));
                        }
                        else
                        {
                            gameManager.InitializeGame("New Game");
                        }
                    });
                    break;
            }
            #endregion

            #endregion

            IsInitializing = false;
            IsInitialized = true;
            Debug.Log($"[LooCast] Finished Initialization in Scene '{activeSceneName}'.");

            LateInitialize();
        }

        private void LateInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLateInitializing = true;
            Debug.Log($"[LooCast] Starting Late Pre-Initialization in Scene '{activeSceneName}'.");

            #region Late Initialization

            #region Internal Managers
            foreach (InternalManager internalManager in internalManagers)
            {
                internalManager.LateInitialize();
            }
            #endregion

            #endregion

            IsLateInitializing = false;
            IsLateInitialized = true;
            Debug.Log($"[LooCast] Finished Late Pre-Initialization in Scene '{activeSceneName}'.");
        }

        private void EarlyPostInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsEarlyPostInitializing = true;
            Debug.Log($"[LooCast] Starting Early Post-Initialization in Scene '{activeSceneName}'.");

            #region Early Post-Initialization

            #region Internal Managers
            foreach (InternalManager internalManager in internalManagers)
            {
                internalManager.EarlyPostInitalize();
            }
            #endregion

            #endregion

            IsEarlyPostInitializing = false;
            IsEarlyPostInitialized = true;
            Debug.Log($"[LooCast] Finished Early Post-Initialization in Scene '{activeSceneName}'.");

            PostInitialize();
        }

        private void PostInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsPostInitializing = true;
            Debug.Log($"[LooCast] Starting Post-Initialization in Scene '{activeSceneName}'.");

            #region Post-Initialization

            #region Internal Managers
            foreach (InternalManager internalManager in internalManagers)
            {
                internalManager.PostInitialize();
            }
            #endregion

            #endregion

            IsPostInitializing = false;
            IsPostInitialized = true;
            Debug.Log($"[LooCast] Finished Post-Initialization in Scene '{activeSceneName}'.");

            LatePostInitialize();
        }

        private void LatePostInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLatePostInitializing = true;
            Debug.Log($"[LooCast] Starting Late Post-Initialization in Scene '{activeSceneName}'.");

            #region Late Post-Initialization

            #region Internal Managers
            foreach (InternalManager internalManager in internalManagers)
            {
                internalManager.LatePostInitialize();
            }
            #endregion

            #endregion

            IsLatePostInitializing = false;
            IsLatePostInitialized = true;
            Debug.Log($"[LooCast] Finished Late Post-Initialization in Scene '{activeSceneName}'.");
        }
        #endregion

        #region Termination Phases
        private void EarlyPreTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsEarlyPreTerminating = true;
            Debug.Log($"[LooCast] Starting Early Pre-Termination in Scene '{activeSceneName}'.");

            #region Early Pre-Termination

            #region Internal Managers
            foreach (InternalManager internalManager in internalManagers.Reverse())
            {
                internalManager.EarlyPreTerminate();
            }
            #endregion

            #endregion

            IsEarlyPreTerminating = false;
            IsEarlyPreTerminated = true;
            Debug.Log($"[LooCast] Finished Early Pre-Termination in Scene '{activeSceneName}'.");

            PreTerminate();
        }

        private void PreTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsPreTerminating = true;
            Debug.Log($"[LooCast] Starting Pre-Termination in Scene '{activeSceneName}'.");

            #region Pre-Termination

            #region Internal Managers
            foreach (InternalManager internalManager in internalManagers.Reverse())
            {
                internalManager.PreTerminate();
            }
            #endregion

            #endregion

            IsPreTerminating = false;
            IsPreTerminated = true;
            Debug.Log($"[LooCast] Finished Pre-Termination in Scene '{activeSceneName}'.");

            LatePreTerminate();
        }

        private void LatePreTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLatePreTerminating = true;
            Debug.Log($"[LooCast] Starting Late Pre-Termination in Scene '{activeSceneName}'.");

            #region Late Pre-Termination

            #region Internal Managers
            foreach (InternalManager internalManager in internalManagers.Reverse())
            {
                internalManager.LatePreTerminate();
            }
            #endregion

            #endregion

            IsLatePreTerminating = false;
            IsLatePreTerminated = true;
            Debug.Log($"[LooCast] Finished Late Pre-Termination in Scene '{activeSceneName}'.");

            EarlyTerminate();
        }

        private void EarlyTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsEarlyTerminating = true;
            Debug.Log($"[LooCast] Starting Early Termination in Scene '{activeSceneName}'.");

            #region Early Termination

            #region Internal Managers
            foreach (InternalManager internalManager in internalManagers.Reverse())
            {
                internalManager.EarlyTerminate();
            }
            #endregion

            #endregion

            IsEarlyTerminating = false;
            IsEarlyTerminated = true;
            Debug.Log($"[LooCast] Finished Early Termination in Scene '{activeSceneName}'.");

            Terminate();
        }

        private void Terminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsTerminating = true;
            Debug.Log($"[LooCast] Starting Termination in Scene '{activeSceneName}'.");

            #region Termination

            #region Internal Managers
            foreach (InternalManager internalManager in internalManagers.Reverse())
            {
                internalManager.Terminate();
            }
            #endregion

            #endregion

            IsTerminating = false;
            IsTerminated = true;
            Debug.Log($"[LooCast] Finished Termination in Scene '{activeSceneName}'.");

            LateTerminate();
        }

        private void LateTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLateTerminating = true;
            Debug.Log($"[LooCast] Starting Late Termination in Scene '{activeSceneName}'.");

            #region Late Termination

            #region Internal Managers
            foreach (InternalManager internalManager in internalManagers.Reverse())
            {
                internalManager.LateTerminate();
            }
            #endregion

            #endregion

            IsLateTerminating = false;
            IsLateTerminated = true;
            Debug.Log($"[LooCast] Finished Late Termination in Scene '{activeSceneName}'.");

            EarlyPostTerminate();
        }

        private void EarlyPostTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsEarlyPostTerminating = true;
            Debug.Log($"[LooCast] Starting Early Post-Termination in Scene '{activeSceneName}'.");

            #region Early Post-Termination

            #region Internal Managers
            foreach (InternalManager internalManager in internalManagers.Reverse())
            {
                internalManager.EarlyPostTerminate();
            }
            #endregion

            #endregion

            IsEarlyPostTerminating = false;
            IsEarlyPostTerminated = true;
            Debug.Log($"[LooCast] Finished Early Post-Termination in Scene '{activeSceneName}'.");

            PostTerminate();
        }

        private void PostTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsPostTerminating = true;
            Debug.Log($"[LooCast] Starting Post-Termination in Scene '{activeSceneName}'.");

            #region Post-Termination

            #region Internal Managers
            foreach (InternalManager internalManager in internalManagers.Reverse())
            {
                internalManager.PostTerminate();
            }
            #endregion

            #endregion

            IsPostTerminating = false;
            IsPostTerminated = true;
            Debug.Log($"[LooCast] Finished Post-Termination in Scene '{activeSceneName}'.");

            LatePostTerminate();
        }

        private void LatePostTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLatePostTerminating = true;
            Debug.Log($"[LooCast] Starting Late Post-Termination in Scene '{activeSceneName}'.");

            #region Late Post-Termination

            #region Internal Managers
            foreach (InternalManager internalManager in internalManagers.Reverse())
            {
                internalManager.LatePostTerminate();
            }
            #endregion

            #endregion

            IsLatePostTerminating = false;
            IsLatePostTerminated = true;
            Debug.Log($"[LooCast] Finished Late Post-Termination in Scene '{activeSceneName}'.");
        }
        #endregion
        
        /// <summary>
        /// Returns the core module managers in the order they should be initialized.
        /// </summary>
        private ICoreModuleManager[] GetCoreModuleManagers()
        {
            return new ICoreModuleManager[]
            {
                // Read the mod folder for valid core module managers and load them
                global::LooCast.Core.CoreManager.Instance
            };
        }
        #endregion

        #region Overrides
        protected override void PreConstruct()
        {
            base.PreConstruct();

            CoreModuleManagers = GetCoreModuleManagers();

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

            // Register all system registries
            // Register all system hierarchies
            // Register everything that's part of the core system in the respective registries and hierarchies
            
            // Pre-Initialize all core module managers
        }

        protected override void Construct()
        {
            base.Construct();

            // Initialize all core module managers
        }

        protected override void PostConstruct()
        {
            base.PostConstruct();

            // Post-Initialize all core module managers
        }
        #endregion
    }
}
