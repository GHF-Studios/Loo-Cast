using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace LooCast.System
{
    using global::LooCast.System.Exceptions;
    using global::LooCast.System.Identifiers;
    using global::LooCast.System.Managers;
    using global::LooCast.System.Registries;
    using global::LooCast.Game;
    using global::LooCast.Scene;
    using global::LooCast.Steamworks;
    using global::LooCast.Util;

    public sealed class MainManager : InternalManager
    {
        #region Static Properties
        public static MainManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[MainManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    UnityEngine.GameObject.DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = LooCast.Instance.gameObject.transform;
                    instance = new MainManager();
                }
                return instance;
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
                return new CoreModuleManager[] 
                {
                    Core.CoreManager.Instance 
                };
            }
        }
        #endregion

        #region Static Fields
        private static MainManager instance;
        #endregion

        #region Properties
        public string RootPersistentPath
        {
            get
            {
                if (string.IsNullOrEmpty(rootPersistentPath))
                {
                    rootPersistentPath = Application.persistentDataPath;
                }
                return rootPersistentPath;
            }
        }
        #endregion

        #region Fields
        private string rootPersistentPath;
        #endregion

        #region Constructors
        public MainManager() : base("LooCast.System:MainManager", null)
        {
            RegisterEarlyPreInitializationAction(() => 
            {
                foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
                {
                    coreModuleManager.EarlyPreInitialize();
                }
            });
            RegisterPreInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
                {
                    coreModuleManager.PreInitialize();
                }
            });
            RegisterLatePreInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
                {
                    coreModuleManager.LatePreInitialize();
                }
            });
            RegisterEarlyInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
                {
                    coreModuleManager.EarlyInitialize();
                }
            });
            RegisterInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
                {
                    coreModuleManager.Initialize();
                }
            });
            RegisterLateInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
                {
                    coreModuleManager.LateInitialize();
                }
            });
            RegisterEarlyPostInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
                {
                    coreModuleManager.EarlyPostInitalize();
                }
            });
            RegisterPostInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
                {
                    coreModuleManager.PostInitialize();
                }
            });
            RegisterLatePostInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
                {
                    coreModuleManager.LatePostInitialize();
                }
            });
        }
        #endregion
    }
}
