using System;
using System.IO;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace LooCast.Core
{
    using Identifier;
    using Registry;
    using LooCast.Data;
    using Mod;
    using Module;
    using Util;
    using Game;
    using Universe;
    
    public class CoreManager : Component, IManager
    {
        #region Static Properties
        public static CoreManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[CoreManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    return instanceObject.AddComponent<CoreManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        public static string ModsFolderPath
        {
            get
            {
                return Path.Combine(Data.Path, "Mods");
            }
        }
        public static Games Games => games;
        public static Game GameToBeLoaded => gameToBeLoaded;    // TODO: Implement this
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
        private static CoreManager instance;
        private static Games games;
        private static Game gameToBeLoaded;
        public static float saveInterval = 30.0f;
        #endregion

        #region Methods
        public void PreInitialize()
        {
            Debug.Log($"[CoreManager] Starting Pre-Initialization.");

            #region Pre-Initialization

            IsPreInitializing = true;

            #region IdentifierManager
            IdentifierManager idManager = IdentifierManager.Instance;
            idManager.Initialize();
            #endregion

            #region RegistryManager
            RegistryManager.Instance.Initialize();
            #endregion

            #region Modding Framework
            try
            {
                ModManager.Instance.Initialize();
            }
            catch (Exception exception)
            {
                Debug.LogError($"[CoreManager] An error occured while initializing the ModManager!");
                Debug.LogException(exception);
                Debug.Log("[CoreManager] Quitting Game.");
                Application.Quit();
            }

            try
            {
                ModuleManager.Instance.Initialize();
            }
            catch (Exception exception)
            {
                Debug.LogError($"[CoreManager] An error occured while initializing the ModuleManager!");
                Debug.LogException(exception);
                Debug.Log("[CoreManager] Quitting Game.");
                Application.Quit();
            }

            #region Callback Invocations
            ModManager.Instance.OnPreInitialize();
            ModuleManager.Instance.OnPreInitialize();
            #endregion

            #endregion

            IsPreInitializing = false;
            IsPreInitialized = true;

            #endregion

            Debug.Log($"[CoreManager] Finished Pre-Initialization.");
            _ = Instance;
        }
        
        public void Initialize()
        {
            Debug.Log($"[CoreManager] Starting Initialization.");

            #region Initialization

            IsInitializing = true;

            #region Data.Path
            _ = Data.Path;
            #endregion

            #region CoreManager
            if (instance != null)
            {
                Destroy(gameObject);
                return;
            }
            instance = this;
            DontDestroyOnLoad(this);

            games = Games.Load();
            #endregion

            #region SteamManager
            _ = SteamManager.Initialized;
            #endregion

            #region Modding Framework

            #region Callback Invocations
            ModManager.Instance.OnInitialize();
            ModuleManager.Instance.OnInitialize();
            #endregion

            #endregion

            #region Utilities
            TimerUtil.InitializeInstance();
            Universe.DensityMapGenerationUtil.InitializeInstance();
            #endregion

            #region Scene
            string activeSceneName = SceneManager.GetActiveScene().name;
            switch (activeSceneName)
            {
                case "MainMenu":
                    break;
                case "Game":
                    GameManager.AddPostInitializationAction(() =>
                    {
                        GameManager gameManager = FindObjectOfType<GameManager>();
                        if (games.Contains("New Game"))
                        {
                            gameManager.InitializeGame(games.GetGame("New Game"));
                        }
                        else
                        {
                            gameManager.InitializeGame("New Game");
                        }
                    });
                    break;
            }
            #endregion

            IsInitializing = false;
            IsInitialized = true;

            #endregion
            
            Debug.Log($"[CoreManager] Finished Initialization.");
        }

        public void PostInitialize()
        {
            Debug.Log($"[CoreManager] Starting Post-Initialization.");

            #region Post-Initialization

            IsPostInitializing = true;
            
            #region Modding Framework

            #region Callback Invocations
            ModManager.Instance.OnPostInitialize();
            ModuleManager.Instance.OnPostInitialize();
            #endregion

            #endregion

            IsPostInitializing = false;
            IsPostInitialized = true;

            #endregion

            Debug.Log($"[CoreManager] Finished Post-Initialization.");
        }
        #endregion
    }
}
