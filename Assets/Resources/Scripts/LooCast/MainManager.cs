using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace LooCast
{
    using LooCast.System;
    using LooCast.System.Identification;
    using LooCast.System.Management;
    using LooCast.Game;
    using LooCast.Scene;
    using LooCast.Steamworks;
    using LooCast.Util;
    using LooCast.Data;
    using LooCast.Universe;

    public class MainManager : MonoBehaviour, INamespaceProvider, ITypeProvider, ISingletonInstanceProvider
    {
        #region Static Properties
        
        #region Initialization Phase Flags
        public static bool IsEarlyPreInitializing { get; private set; }
        public static bool IsPreInitializing { get; private set; }
        public static bool IsLatePreInitializing { get; private set; }
        public static bool IsEarlyPreInitialized { get; private set; }
        public static bool IsPreInitialized { get; private set; }
        public static bool IsLatePreInitialized { get; private set; }

        public static bool IsEarlyInitializing { get; private set; }
        public static bool IsInitializing { get; private set; }
        public static bool IsLateInitializing { get; private set; }
        public static bool IsEarlyInitialized { get; private set; }
        public static bool IsInitialized { get; private set; }
        public static bool IsLateInitialized { get; private set; }

        public static bool IsEarlyPostInitializing { get; private set; }
        public static bool IsPostInitializing { get; private set; }
        public static bool IsLatePostInitializing { get; private set; }
        public static bool IsEarlyPostInitialized { get; private set; }
        public static bool IsPostInitialized { get; private set; }
        public static bool IsLatePostInitialized { get; private set; }

        public static bool IsFullyPreInitialized
        {
            get
            {
                return IsEarlyPreInitialized && IsPreInitialized && IsLatePreInitialized;
            }
        }
        public static bool IsFullyInitialized
        {
            get
            {
                return IsEarlyInitialized && IsInitialized && IsLateInitialized;
            }
        }
        public static bool IsFullyPostInitialized
        {
            get
            {
                return IsEarlyPostInitialized && IsPostInitialized && IsLatePostInitialized;
            }
        }
        public static bool IsCompletelyInitialized
        {
            get
            {
                return IsFullyPreInitialized && IsFullyInitialized && IsPostInitialized;
            }
        }
        #endregion

        #region Termination Phase Flags
        public static bool IsEarlyPreTerminating { get; private set; }
        public static bool IsPreTerminating { get; private set; }
        public static bool IsLatePreTerminating { get; private set; }
        public static bool IsEarlyPreTerminated { get; private set; }
        public static bool IsPreTerminated { get; private set; }
        public static bool IsLatePreTerminated { get; private set; }

        public static bool IsEarlyTerminating { get; private set; }
        public static bool IsTerminating { get; private set; }
        public static bool IsLateTerminating { get; private set; }
        public static bool IsEarlyTerminated { get; private set; }
        public static bool IsTerminated { get; private set; }
        public static bool IsLateTerminated { get; private set; }

        public static bool IsEarlyPostTerminating { get; private set; }
        public static bool IsPostTerminating { get; private set; }
        public static bool IsLatePostTerminating { get; private set; }
        public static bool IsEarlyPostTerminated { get; private set; }
        public static bool IsPostTerminated { get; private set; }
        public static bool IsLatePostTerminated { get; private set; }

        public static bool IsFullyPreTerminated
        {
            get
            {
                return IsEarlyPreTerminated && IsPreTerminated && IsLatePreTerminated;
            }
        }
        public static bool IsFullyTerminated
        {
            get
            {
                return IsEarlyTerminated && IsTerminated && IsLateTerminated;
            }
        }
        public static bool IsFullyPostTerminated
        {
            get
            {
                return IsEarlyPostTerminated && IsPostTerminated && IsLatePostTerminated;
            }
        }
        public static bool IsCompletelyTerminated
        {
            get
            {
                return IsFullyPreTerminated && IsFullyTerminated && IsPostTerminated;
            }
        }
        #endregion

        public static MainManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[MainManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    return instanceObject.AddComponent<MainManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        /// <summary>
        /// All InternalManagers, ordered by their Dependencies(index 0 is RegistryManager, 1 is NamespaceManager, 2 is TypeManager, 3 is InstanceManager, etc.).
        /// </summary>
        public static InternalManager[] InternalManagers
        {
            get
            {
                return new InternalManager[]
                {
                    RegistryManager.Instance,
                    NamespaceManager.Instance,
                    TypeManager.Instance,
                    InstanceManager.Instance,
                };
            }
        }
        /// <summary>
        /// All CoreModuleManagers, ordered by their Dependencies(index 0 is Base Mod Core Module Manager, 1 is Mod Core Module Manager, 2 is Mod Extension Core Module Manager, 3 is Mod Extension Extension Core Module Manager, etc.).
        /// </summary>
        public static CoreModuleManager[] CoreModuleManagers
        {
            get
            {
                // TODO: Implement loading/deserialization/injection of CoreModuleManagers.
            }
        }
        #endregion

        #region Static Fields
        private static MainManager instance;
        public static float saveInterval = 30.0f;
        #endregion

        #region Properties
        public Namespace LooCastNamespace => looCastNamespace;
        public Type LooCastType => looCastType;
        public CSharpInstance LooCastUnityInstance => looCastInstance;
        #endregion

        #region Fields
        private Namespace looCastNamespace;
        private Type looCastType;
        private CSharpInstance looCastInstance;
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

        #region Initialization Phases
        private void EarlyPreInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            
            #region Internal Managers Setup
            #region Pre-Initialization
            Debug.Log($"[MainManager] Pre-Initializing internal manager instances.");
            foreach (InternalManager internalManager in InternalManagers)
            {
                internalManager.PreInitializeInstance();
            }
            Debug.Log($"[MainManager] Pre-Initialized internal manager instances.");
            #endregion

            #region Initialization
            Debug.Log($"[MainManager] Initializing internal manager instances.");
            foreach (InternalManager internalManager in InternalManagers)
            {
                internalManager.InitializeInstance();
            }
            Debug.Log($"[MainManager] Initialized internal manager instances.");
            #endregion

            NamespaceManager namespaceManager = NamespaceManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            UnityInstanceManager unityInstanceManager = UnityInstanceManager.Instance;
            
            looCastNamespace = namespaceManager.GetNamespace("LooCast");
            looCastType = new Type(typeof(MainManager), looCastNamespace);
            looCastInstance = new UnityInstance(this, looCastType);
            
            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            // TODO: Register everything, that's internal

            #region Post-Initialization
            Debug.Log($"[MainManager] Post-Initializing internal manager instances.");
            foreach (InternalManager internalManager in InternalManagers)
            {
                internalManager.PostInitializeInstance();
            }
            Debug.Log($"[MainManager] Post-Initialized internal manager instances.");
            #endregion

            #endregion

            #region Core Module Managers Setup
            
            #region Pre-Initialization
            Debug.Log($"[MainManager] Pre-Initializing core module manager instances.");
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.PreInitializeInstance();
            }
            Debug.Log($"[MainManager] Pre-Initialized core module manager instances.");
            #endregion

            #region Initialization
            Debug.Log($"[MainManager] Initializing core module manager instances.");
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.InitializeInstance();
            }
            Debug.Log($"[MainManager] Initialized core module manager instances.");
            #endregion

            #region Post-Initialization
            Debug.Log($"[MainManager] Post-Initializing core module manager instances.");
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.PostInitializeInstance();
            }
            Debug.Log($"[MainManager] Post-Initialized core module manager instances.");
            #endregion

            #endregion

            IsEarlyPreInitializing = true;
            Debug.Log($"[MainManager] Starting Early Pre-Initialization in Scene '{activeSceneName}'.");

            #region Early Pre-Initialization

            #region Main Manager

            #endregion

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.EarlyPreInitialize();
            }
            #endregion

            #endregion

            IsEarlyPreInitializing = false;
            IsEarlyPreInitialized = true;
            Debug.Log($"[MainManager] Finished Early Pre-Initialization in Scene '{activeSceneName}'.");

            PreInitialize();
        }

        private void PreInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsPreInitializing = true;
            Debug.Log($"[MainManager] Starting Pre-Initialization in Scene '{activeSceneName}'.");

            #region Pre-Initialization

            #region Main Manager

            #endregion

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.PreInitialize();
            }
            #endregion

            #endregion

            IsPreInitializing = false;
            IsPreInitialized = true;
            Debug.Log($"[MainManager] Finished Pre-Initialization in Scene '{activeSceneName}'.");

            LatePreInitialize();
        }

        private void LatePreInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLatePreInitializing = true;
            Debug.Log($"[MainManager] Starting Pre-Initialization in Scene '{activeSceneName}'.");

            #region Late Pre-Initialization

            #region MainManager
            #endregion

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.LatePreInitialize();
            }
            #endregion

            #endregion

            IsPreInitializing = false;
            IsPreInitialized = true;
            Debug.Log($"[MainManager] Finished Pre-Initialization in Scene '{activeSceneName}'.");

            _ = Instance;
        }

        private void EarlyInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsEarlyInitializing = true;
            Debug.Log($"[MainManager] Starting Early Pre-Initialization in Scene '{activeSceneName}'.");

            #region Early Initialization

            #region Main Manager

            #endregion

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.EarlyInitialize();
            }
            #endregion

            #endregion

            IsEarlyInitializing = false;
            IsEarlyInitialized = true;
            Debug.Log($"[MainManager] Finished Early Pre-Initialization in Scene '{activeSceneName}'.");

            Initialize();
        }

        private void Initialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsInitializing = true;
            Debug.Log($"[MainManager] Starting Initialization in Scene '{activeSceneName}'.");

            #region Initialization

            #region MainManager
            #endregion

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.Initialize();
            }
            #endregion

            #region DEPRECATED! [TO BE MOVED!]

            #region SteamManager
            _ = SteamworksManager.Initialized;
            #endregion

            #region Data.Path
            _ = Data.Data.Path;
            #endregion

            #region TimerUtil
            TimerUtil.InitializeInstance();
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

            #endregion

            IsInitializing = false;
            IsInitialized = true;
            Debug.Log($"[MainManager] Finished Initialization in Scene '{activeSceneName}'.");

            LateInitialize();
        }

        private void LateInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLateInitializing = true;
            Debug.Log($"[MainManager] Starting Late Pre-Initialization in Scene '{activeSceneName}'.");

            #region Late Initialization

            #region Main Manager

            #endregion

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.LateInitialize();
            }
            #endregion

            #endregion

            IsLateInitializing = false;
            IsLateInitialized = true;
            Debug.Log($"[MainManager] Finished Late Pre-Initialization in Scene '{activeSceneName}'.");
        }

        private void EarlyPostInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsEarlyPostInitializing = true;
            Debug.Log($"[MainManager] Starting Early Post-Initialization in Scene '{activeSceneName}'.");

            #region Early Post-Initialization

            #region MainManager

            #endregion

            #region Core Module Managers
            LooCast.Core.CoreManager looCastCoreManager = LooCast.Core.CoreManager.Instance;
            looCastCoreManager.PostInitialize();
            #endregion

            #endregion

            IsEarlyPostInitializing = false;
            IsEarlyPostInitialized = true;
            Debug.Log($"[MainManager] Finished Early Post-Initialization in Scene '{activeSceneName}'.");

            PostInitialize();
        }

        private void PostInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsPostInitializing = true;
            Debug.Log($"[MainManager] Starting Post-Initialization in Scene '{activeSceneName}'.");

            #region Post-Initialization

            #region MainManager

            #endregion

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.PostInitialize();
            }
            #endregion

            #endregion

            IsPostInitializing = false;
            IsPostInitialized = true;
            Debug.Log($"[MainManager] Finished Post-Initialization in Scene '{activeSceneName}'.");

            LatePostInitialize();
        }

        private void LatePostInitialize()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLatePostInitializing = true;
            Debug.Log($"[MainManager] Starting Late Post-Initialization in Scene '{activeSceneName}'.");

            #region Late Post-Initialization

            #region MainManager

            #endregion

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.LatePostInitialize();
            }
            #endregion

            #endregion

            IsLatePostInitializing = false;
            IsLatePostInitialized = true;
            Debug.Log($"[MainManager] Finished Late Post-Initialization in Scene '{activeSceneName}'.");
        }
        #endregion

        #region Termination Phases
        private void EarlyPreTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsEarlyPreTerminating = true;
            Debug.Log($"[MainManager] Starting Early Pre-Termination in Scene '{activeSceneName}'.");

            #region Early Pre-Termination

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers.Reverse())
            {
                coreModuleManager.EarlyPreTerminate();
            }
            #endregion

            #region MainManager
            #endregion

            #endregion

            IsEarlyPreTerminating = false;
            IsEarlyPreTerminated = true;
            Debug.Log($"[MainManager] Finished Early Pre-Termination in Scene '{activeSceneName}'.");

            PreTerminate();
        }

        private void PreTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsPreTerminating = true;
            Debug.Log($"[MainManager] Starting Pre-Termination in Scene '{activeSceneName}'.");

            #region Pre-Termination

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers.Reverse())
            {
                coreModuleManager.PreTerminate();
            }
            #endregion

            #region MainManager
            #endregion

            #endregion

            IsPreTerminating = false;
            IsPreTerminated = true;
            Debug.Log($"[MainManager] Finished Pre-Termination in Scene '{activeSceneName}'.");

            LatePreTerminate();
        }

        private void LatePreTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLatePreTerminating = true;
            Debug.Log($"[MainManager] Starting Late Pre-Termination in Scene '{activeSceneName}'.");

            #region Late Pre-Termination

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers.Reverse())
            {
                coreModuleManager.LatePreTerminate();
            }
            #endregion

            #region MainManager
            #endregion

            #endregion

            IsLatePreTerminating = false;
            IsLatePreTerminated = true;
            Debug.Log($"[MainManager] Finished Late Pre-Termination in Scene '{activeSceneName}'.");

            EarlyTerminate();
        }

        private void EarlyTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsEarlyTerminating = true;
            Debug.Log($"[MainManager] Starting Early Termination in Scene '{activeSceneName}'.");

            #region Early Termination

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers.Reverse())
            {
                coreModuleManager.EarlyTerminate();
            }
            #endregion

            #region MainManager
            #endregion

            #endregion

            IsEarlyTerminating = false;
            IsEarlyTerminated = true;
            Debug.Log($"[MainManager] Finished Early Termination in Scene '{activeSceneName}'.");

            Terminate();
        }

        private void Terminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsTerminating = true;
            Debug.Log($"[MainManager] Starting Termination in Scene '{activeSceneName}'.");

            #region Termination

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers.Reverse())
            {
                coreModuleManager.Terminate();
            }
            #endregion

            #region MainManager
            #endregion

            #endregion

            IsTerminating = false;
            IsTerminated = true;
            Debug.Log($"[MainManager] Finished Termination in Scene '{activeSceneName}'.");

            LateTerminate();
        }

        private void LateTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLateTerminating = true;
            Debug.Log($"[MainManager] Starting Late Termination in Scene '{activeSceneName}'.");

            #region Late Termination

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers.Reverse())
            {
                coreModuleManager.LateTerminate();
            }
            #endregion

            #region MainManager
            #endregion

            #endregion

            IsLateTerminating = false;
            IsLateTerminated = true;
            Debug.Log($"[MainManager] Finished Late Termination in Scene '{activeSceneName}'.");

            EarlyPostTerminate();
        }

        private void EarlyPostTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsEarlyPostTerminating = true;
            Debug.Log($"[MainManager] Starting Early Post-Termination in Scene '{activeSceneName}'.");

            #region Early Post-Termination

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers.Reverse())
            {
                coreModuleManager.EarlyPostTerminate();
            }
            #endregion

            #region MainManager
            #endregion

            #endregion

            IsEarlyPostTerminating = false;
            IsEarlyPostTerminated = true;
            Debug.Log($"[MainManager] Finished Early Post-Termination in Scene '{activeSceneName}'.");

            PostTerminate();
        }

        private void PostTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsPostTerminating = true;
            Debug.Log($"[MainManager] Starting Post-Termination in Scene '{activeSceneName}'.");

            #region Post-Termination

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers.Reverse())
            {
                coreModuleManager.PostTerminate();
            }
            #endregion

            #region MainManager
            #endregion

            #endregion

            IsPostTerminating = false;
            IsPostTerminated = true;
            Debug.Log($"[MainManager] Finished Post-Termination in Scene '{activeSceneName}'.");

            LatePostTerminate();
        }

        private void LatePostTerminate()
        {
            string activeSceneName = UnityEngine.SceneManagement.SceneManager.GetActiveScene().name;
            IsLatePostTerminating = true;
            Debug.Log($"[MainManager] Starting Late Post-Termination in Scene '{activeSceneName}'.");

            #region Late Post-Termination

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers.Reverse())
            {
                coreModuleManager.LatePostTerminate();
            }
            #endregion

            #region MainManager
            #endregion

            #endregion

            IsLatePostTerminating = false;
            IsLatePostTerminated = true;
            Debug.Log($"[MainManager] Finished Late Post-Termination in Scene '{activeSceneName}'.");
        }
        #endregion

        #endregion
    }
}
