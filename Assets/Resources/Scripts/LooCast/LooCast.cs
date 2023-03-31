using UnityEngine;

namespace LooCast
{
    using global::LooCast.System;
    using global::LooCast.System.Managers;
    using global::LooCast.Steamworks;

    public sealed class LooCast : MonoBehaviour
    {
        #region Static Properties
        public static LooCast Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[LooCast]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    return instanceObject.AddComponent<LooCast>();
                }
                else
                {
                    return instance;
                }
            }
        }

        /// <summary>
        /// All internalManagers, ordered by their Dependencies(index 0 is RegistryManager, 1 is NamespaceManager, 2 is TypeManager, 3 is InstanceManager, etc.).
        /// </summary>
        private InternalManager[] internalManagers;

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
        #endregion

        #region Static Fields
        private static LooCast instance;
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

        #endregion
    }
}
