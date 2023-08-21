using System;
using System.IO;
using System.Collections.Generic;
using System.Reflection;
using UnityEngine;

namespace LooCast
{
    using Core;
    
    public class MainManager : Component
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
                    return instanceObject.AddComponent<MainManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        public static bool IsPreInitializing { get; private set; }
        public static bool IsPreInitialized { get; private set; }
        public static bool IsInitializing { get; private set; }
        public static bool IsInitialized { get; private set; }
        public static bool IsPostInitializing { get; private set; }
        public static bool IsPostInitialized { get; private set; }
        public static bool IsFullyInitialized
        {
            get
            {
                return IsPreInitialized && IsInitialized && IsPostInitialized;
            }
        }
        #endregion

        #region Static Fields
        private static MainManager instance;
        private static List<IEntrypoint> modEntrypoints;
        #endregion

        #region Unity Callbacks
        private void Awake()
        {
            IsInitializing = true;

            #region Initialization
            if (instance != null)
            {
                Destroy(gameObject);
                return;
            }
            instance = this;
            DontDestroyOnLoad(this);

            foreach (IEntrypoint modEntrypoint in modEntrypoints)
            {
                modEntrypoint.Initialize();
            }
            #endregion

            IsInitializing = false;
            IsInitialized = true;
        }
        #endregion

        #region Static Methods
        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
        private static void PreInitialize()
        {
            IsPreInitializing = true;

            #region Pre-Initialization
            modEntrypoints = new List<IEntrypoint>();
            string modsFolderPath = Path.Combine(Application.dataPath, "Data", "Mods");
            string[] modDirectoryPaths = Directory.GetDirectories(modsFolderPath);
            foreach (string modDirectoryPath in modDirectoryPaths)
            {
                DirectoryInfo modFolder = new DirectoryInfo(modDirectoryPath);
                string modName = modFolder.Name;
                string modEntrypointPath = Path.Combine(modFolder.FullName, modName + "Entrypoint.cs");
                if (File.Exists(modEntrypointPath))
                {
                    Assembly modAssembly = Assembly.LoadFile(modEntrypointPath);
                    Type entrypointType = modAssembly.GetType(modName + ".Entrypoint");
                    IEntrypoint modEntrypoint = (IEntrypoint)Activator.CreateInstance(entrypointType);
                    modEntrypoints.Add(modEntrypoint);
                }
            }

            foreach (IEntrypoint modEntrypoint in modEntrypoints)
            {
                modEntrypoint.PreInitialize();
            }
            #endregion

            IsPreInitializing = false;
            IsPreInitialized = true;
            _ = Instance;
        }

        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.AfterSceneLoad)]
        private static void PostInitialize()
        {
            IsPostInitializing = true;

            #region Post-Initialization
            foreach (IEntrypoint modEntrypoint in modEntrypoints)
            {
                modEntrypoint.PostInitialize();
            }
            #endregion

            IsPostInitializing = false;
            IsPostInitialized = true;
        }
        #endregion
    }
}
