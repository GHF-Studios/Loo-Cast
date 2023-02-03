using System;
using System.IO;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace LooCast
{
    using Game;
    using Util;
    using Universe;
    using UI.Screen;
    using LooCast.Data;
    using Mod;
    using Module;
    using Registry;
    using Identifier;
    using System.Reflection;

    public class MainManager : Component
    {
        #region Static Properties
        public static MainManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new UnityEngine.GameObject("[MainManager]");
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
        private static List<Entrypoint> modEntrypoints;
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

            foreach (Entrypoint modEntrypoint in modEntrypoints)
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
            modEntrypoints = new List<Entrypoint>();
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
                    Entrypoint modEntrypoint = (Entrypoint)Activator.CreateInstance(entrypointType);
                    modEntrypoints.Add(modEntrypoint);
                }
            }

            foreach (Entrypoint modEntrypoint in modEntrypoints)
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
            foreach (Entrypoint modEntrypoint in modEntrypoints)
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
