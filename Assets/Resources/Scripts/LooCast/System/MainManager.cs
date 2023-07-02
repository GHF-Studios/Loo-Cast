using UnityEngine;
using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public sealed class MainManager : Folder, IManager
    {
        #region Static Properties
        public static MainManager Instance
        {
            get
            {
                if (instance == null)
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
        public string ManagerName => "MainManager";
        public ExtendedMonoBehaviour ManagerMonoBehaviour => MainManagerMonoBehaviour.Instance;
        public ICoreModuleManager[] CoreModuleManagers { get; private set; }
        IManager IChild<IManager>.Parent => null;
        IEnumerable<IManager> IParent<IManager>.Children => (IEnumerable<IManager>)FolderChildren;
        
        #region Initialization Phase Flags
        public bool IsEarlyPreInitializing { get; private set; }
        public bool IsEarlyPreInitialized { get; private set; }
        public bool IsPreInitializing { get; private set; }
        public bool IsPreInitialized { get; private set; }
        public bool IsLatePreInitializing { get; private set; }
        public bool IsLatePreInitialized { get; private set; }

        public bool IsEarlyInitializing { get; private set; }
        public bool IsEarlyInitialized { get; private set; }
        public bool IsInitializing { get; private set; }
        public bool IsInitialized { get; private set; }
        public bool IsLateInitializing { get; private set; }
        public bool IsLateInitialized { get; private set; }

        public bool IsEarlyPostInitializing { get; private set; }
        public bool IsEarlyPostInitialized { get; private set; }
        public bool IsPostInitializing { get; private set; }
        public bool IsPostInitialized { get; private set; }
        public bool IsLatePostInitializing { get; private set; }
        public bool IsLatePostInitialized { get; private set; }

        public bool IsFullyPreInitializing
        {
            get
            {
                return IsEarlyPreInitializing || IsPreInitializing || IsLatePreInitializing;
            }
        }
        public bool IsFullyPreInitialized
        {
            get
            {
                return IsEarlyPreInitialized && IsPreInitialized && IsLatePreInitialized;
            }
        }
        public bool IsFullyInitializing
        {
            get
            {
                return IsEarlyInitializing || IsInitializing || IsLateInitializing;
            }
        }
        public bool IsFullyInitialized
        {
            get
            {
                return IsEarlyInitialized && IsInitialized && IsLateInitialized;
            }
        }
        public bool IsFullyPostInitializing
        {
            get
            {
                return IsEarlyPostInitializing || IsPostInitializing || IsLatePostInitializing;
            }
        }
        public bool IsFullyPostInitialized
        {
            get
            {
                return IsEarlyPostInitialized && IsPostInitialized && IsLatePostInitialized;
            }
        }
        public bool IsCompletelyInitializing
        {
            get
            {
                return IsFullyPreInitializing || IsFullyInitializing || IsFullyPostInitializing;
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
        public bool IsEarlyPreTerminating { get; private set; }
        public bool IsPreTerminating { get; private set; }
        public bool IsLatePreTerminating { get; private set; }
        public bool IsEarlyPreTerminated { get; private set; }
        public bool IsPreTerminated { get; private set; }
        public bool IsLatePreTerminated { get; private set; }

        public bool IsEarlyTerminating { get; private set; }
        public bool IsTerminating { get; private set; }
        public bool IsLateTerminating { get; private set; }
        public bool IsEarlyTerminated { get; private set; }
        public bool IsTerminated { get; private set; }
        public bool IsLateTerminated { get; private set; }

        public bool IsEarlyPostTerminating { get; private set; }
        public bool IsPostTerminating { get; private set; }
        public bool IsLatePostTerminating { get; private set; }
        public bool IsEarlyPostTerminated { get; private set; }
        public bool IsPostTerminated { get; private set; }
        public bool IsLatePostTerminated { get; private set; }

        public bool IsFullyPreTerminating
        {
            get
            {
                return IsEarlyPreTerminating || IsPreTerminating || IsLatePreTerminating;
            }
        }
        public bool IsFullyPreTerminated
        {
            get
            {
                return IsEarlyPreTerminated && IsPreTerminated && IsLatePreTerminated;
            }
        }
        public bool IsFullyTerminating
        {
            get
            {
                return IsEarlyTerminating || IsTerminating || IsLateTerminating;
            }
        }
        public bool IsFullyTerminated
        {
            get
            {
                return IsEarlyTerminated && IsTerminated && IsLateTerminated;
            }
        }
        public bool IsFullyPostTerminating
        {
            get
            {
                return IsEarlyPostTerminating || IsPostTerminating || IsLatePostTerminating;
            }
        }
        public bool IsFullyPostTerminated
        {
            get
            {
                return IsEarlyPostTerminated && IsPostTerminated && IsLatePostTerminated;
            }
        }
        public bool IsCompletelyTerminating
        {
            get
            {
                return IsFullyPreTerminating || IsFullyTerminating || IsFullyPostTerminating;
            }
        }
        public bool IsCompletelyTerminated
        {
            get
            {
                return IsFullyPreTerminated && IsFullyTerminated && IsPostTerminated;
            }
        }

        public bool IsFullyPostTermináted => throw new NotImplementedException();

        public bool IsCompletelyPreTerminating => throw new NotImplementedException();

        public bool IsCompletelyPreTerminated => throw new NotImplementedException();
        #endregion

        #endregion

        #region Constructors
        private MainManager() : base()
        {
            CoreModuleManagers = new ICoreModuleManager[]
            {
                // TODO:    Read the mod hierarchyFolder for valid core module managers and load them.
                //          This process is internal to the MainManager and thus there are no Methods to manage the child managers.
                // LooCast.Core.CoreManager.Instance,
                // ThermalDynamics.Core.CoreManager.Instance,
                // ThermalExpansion.Core.CoreManager.Instance,
                // CrazySexMod.Core.CoreManager.Instance,
                // CocaineMod.Core.CoreManager.Instance,
                // PineappleMod.Core.CoreManager.Instance
            };
            
            Folder looCastFolder = new Folder("LooCast", this);
            
            Folder looCastSystemFolder = new Folder("System", looCastFolder);
            Folder looCastSystemPathsFolder = new Folder("Paths", looCastSystemFolder);
            
            Folder looCastSystem32Folder = new Folder("System32", looCastFolder);
            Folder looCastSystem32LibrariesFolder = new Folder("Libraries", looCastSystem32Folder);

            Folder looCastAppDataFolder = new Folder("AppData", looCastFolder);
            Folder looCastAppDataRoamingFolder = new Folder("Roaming", looCastAppDataFolder);
        }
        #endregion

        #region Methods
        public void OnPreAwake()
        {
            EarlyPreInitialize();
            PreInitialize();
            LatePreInitialize();
        }

        public void OnAwake()
        {
            EarlyInitialize();
            Initialize();
            LateInitialize();
        }

        public void OnPostAwake()
        {
            EarlyPostInitialize();
            PostInitialize();
            LatePostInitialize();
        }

        public void OnDisable()
        {
            if (!IsEarlyPreTerminating && !IsPreTerminating && !IsLatePreTerminating && !IsEarlyTerminating && !IsTerminating && !IsLateTerminating && !IsEarlyPostTerminating && !IsPostTerminating && !IsLatePostTerminating)
            {
                EarlyPreTerminate();
                PreTerminate();
                LatePreTerminate();
                EarlyTerminate();
                Terminate();
                LateTerminate();
                EarlyPostTerminate();
                PostTerminate();
                LatePostTerminate();
            }
        }

        public void OnApplicationQuit()
        {
            if (!IsEarlyPreTerminating && !IsPreTerminating && !IsLatePreTerminating && !IsEarlyTerminating && !IsTerminating && !IsLateTerminating && !IsEarlyPostTerminating && !IsPostTerminating && !IsLatePostTerminating)
            {
                EarlyPreTerminate();
                PreTerminate();
                LatePreTerminate();
                EarlyTerminate();
                Terminate();
                LateTerminate();
                EarlyPostTerminate();
                PostTerminate();
                LatePostTerminate();
            }
        }
        
        #region Initialization Phases
        public void EarlyPreInitialize()
        {
            IsEarlyPreInitializing = true;
            Debug.Log($"[MainManager] Starting Early Pre-Initialization.");

            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.EarlyPreInitialize();
            }

            Debug.Log($"[MainManager] Finished Early Pre-Initialization.");
            IsEarlyPreInitializing = false;
            IsEarlyPreInitialized = true;
        }

        public void PreInitialize()
        {
            IsPreInitializing = true;
            Debug.Log($"[MainManager] Starting Pre-Initialization.");

            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.PreInitialize();
            }

            Debug.Log($"[MainManager] Finished Pre-Initialization.");
            IsPreInitializing = false;
            IsPreInitialized = true;
        }

        public void LatePreInitialize()
        {
            IsLatePreInitializing = true;
            Debug.Log($"[MainManager] Starting Late Pre-Initialization.");

            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.LatePreInitialize();
            }

            Debug.Log($"[MainManager] Finished Late Pre-Initialization.");
            IsLatePreInitializing = false;
            IsLatePreInitialized = true;
        }

        public void EarlyInitialize()
        {
            IsEarlyInitializing = true;
            Debug.Log($"[MainManager] Starting Early Initialization.");

            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.EarlyInitialize();
            }

            Debug.Log($"[MainManager] Finished Early Initialization.");
            IsEarlyInitializing = false;
            IsEarlyInitialized = true;
        }

        public void Initialize()
        {
            IsInitializing = true;
            Debug.Log($"[MainManager] Starting Initialization.");

            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.Initialize();
            }

            Debug.Log($"[MainManager] Finished Initialization.");
            IsInitializing = false;
            IsInitialized = true;
        }

        public void LateInitialize()
        {
            IsLateInitializing = true;
            Debug.Log($"[MainManager] Starting Late Initialization.");

            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.LateInitialize();
            }

            Debug.Log($"[MainManager] Finished Late Initialization.");
            IsLateInitializing = false;
            IsLateInitialized = true;
        }

        public void EarlyPostInitialize()
        {
            IsEarlyPostInitializing = true;
            Debug.Log($"[MainManager] Starting Early Post-Initialization.");

            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.EarlyPostInitialize();
            }

            Debug.Log($"[MainManager] Finished Early Post-Initialization.");
            IsEarlyPostInitializing = false;
            IsEarlyPostInitialized = true;
        }

        public void PostInitialize()
        {
            IsPostInitializing = true;
            Debug.Log($"[MainManager] Starting Post-Initialization.");

            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.PostInitialize();
            }

            Debug.Log($"[MainManager] Finished Post-Initialization.");
            IsPostInitializing = false;
            IsPostInitialized = true;
        }

        public void LatePostInitialize()
        {
            IsLatePostInitializing = true;
            Debug.Log($"[MainManager] Starting Late Post-Initialization.");

            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.LatePostInitialize();
            }

            Debug.Log($"[MainManager] Finished Late Post-Initialization.");
            IsLatePostInitializing = false;
            IsLatePostInitialized = true;
        }
        #endregion

        #region Termination Phases
        public void EarlyPreTerminate()
        {
            IsEarlyPreTerminating = true;
            Debug.Log($"[MainManager] Starting Early Pre-Termination.");

            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.EarlyPreTerminate();
            }

            Debug.Log($"[MainManager] Finished Early Pre-Termination.");
            IsEarlyPreTerminating = false;
            IsEarlyPreTerminated = true;
        }

        public void PreTerminate()
        {
            IsPreTerminating = true;
            Debug.Log($"[MainManager] Starting Pre-Termination.");

            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.PreTerminate();
            }

            Debug.Log($"[MainManager] Finished Pre-Termination.");
            IsPreTerminating = false;
            IsPreTerminated = true;
        }

        public void LatePreTerminate()
        {
            IsLatePreTerminating = true;
            Debug.Log($"[MainManager] Starting Late Pre-Termination.");

            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.LatePreTerminate();
            }

            Debug.Log($"[MainManager] Finished Late Pre-Termination.");
            IsLatePreTerminating = false;
            IsLatePreTerminated = true;
        }

        public void EarlyTerminate()
        {
            IsEarlyTerminating = true;
            Debug.Log($"[MainManager] Starting Early Termination.");

            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.EarlyTerminate();
            }

            Debug.Log($"[MainManager] Finished Early Termination.");
            IsEarlyTerminating = false;
            IsEarlyTerminated = true;
        }

        public void Terminate()
        {
            IsTerminating = true;
            Debug.Log($"[MainManager] Starting Termination.");

            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.Terminate();
            }

            Debug.Log($"[MainManager] Finished Termination.");
            IsTerminating = false;
            IsTerminated = true;
        }

        public void LateTerminate()
        {
            IsLateTerminating = true;
            Debug.Log($"[MainManager] Starting Late Termination.");

            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.LateTerminate();
            }

            Debug.Log($"[MainManager] Finished Late Termination.");
            IsLateTerminating = false;
            IsLateTerminated = true;
        }

        public void EarlyPostTerminate()
        {
            IsEarlyPostTerminating = true;
            Debug.Log($"[MainManager] Starting Early Post-Termination.");

            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.EarlyPostTerminate();
            }

            Debug.Log($"[MainManager] Finished Early Post-Termination.");
            IsEarlyPostTerminating = false;
            IsEarlyPostTerminated = true;
        }

        public void PostTerminate()
        {
            IsPostTerminating = true;
            Debug.Log($"[MainManager] Starting Post-Termination.");

            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.PostTerminate();
            }

            Debug.Log($"[MainManager] Finished Post-Termination.");
            IsPostTerminating = false;
            IsPostTerminated = true;
        }

        public void LatePostTerminate()
        {
            IsLatePostTerminating = true;
            Debug.Log($"[MainManager] Starting Late Post-Termination.");

            foreach (ICoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.LatePostTerminate();
            }

            Debug.Log($"[MainManager] Finished Late Post-Termination.");
            IsLatePostTerminating = false;
            IsLatePostTerminated = true;
        }
        #endregion

        #endregion
    }
}
