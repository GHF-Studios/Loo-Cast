using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace LooCast.Core
{
    using AI;
    using Ally;
    using AOE;
    using Arc;
    using Asteroid;
    using Attribute;
    using Attribute.Stat;
    using Background;
    using Chance;
    using Core.Registry;
    using Currency;
    using Data;
    using Diagnostic;
    using Enemy;
    using Event;
    using Experience;
    using Game;
    using Generator;
    using Health;
    using Identifier;
    using Indicator;
    using Inventory;
    using Item;
    using MainMenu;
    using Math;
    using Math.Map;
    using Mission;
    using Mission.Reward;
    using Mission.Target;
    using Mission.Task;
    using Mission.Trigger;
    using Movement;
    using Movement.Effect;
    using Noise;
    using Observer;
    using Orb;
    using Particle;
    using Player;
    using Projectile;
    using Random;
    using Resource;
    using Sound;
    using Spawner;
    using StateMachine;
    using Station;
    using Statistic;
    using Steamworks;
    using Target;
    using Test;
    using UI;
    using UI.Bar;
    using UI.Button;
    using UI.Canvas;
    using UI.Cursor;
    using UI.HUD;
    using UI.Inspector;
    using UI.Inventory;
    using UI.Level;
    using UI.Panel;
    using UI.Reward;
    using UI.Screen;
    using UI.Slider;
    using UI.Tab;
    using UI.Task;
    using UI.Timer;
    using UI.Title;
    using UI.Value;
    using Universe;
    using Util;
    using Util.Collections;
    using Variable;

    public class MainManager : MonoBehaviour
    {
        #region Enums
        public enum SceneType
        {
            MainMenu,
            Game
        }
        #endregion

        #region Static Properties
        public static MainManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[MainManager]");
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
        private static MainManager instance;
        private static Games games;
        private static Game gameToBeLoaded;
        public static float saveInterval = 30.0f;
        #endregion

        #region Unity Callbacks
        private void Awake()
        {
            string activeSceneName = SceneManager.GetActiveScene().name;
            Debug.Log($"[MainManager] Starting Initialization in Scene '{activeSceneName}'.");

            IsInitializing = true;

            #region Initialization

            #region MainManager Initialization
            if (instance != null)
            {
                Destroy(gameObject);
                return;
            }
            instance = this;
            DontDestroyOnLoad(this);

            games = Games.Load();

            Debug.Log($"[MainManager] Initialized.");
            #endregion

            #region SteamManager Initialization
            _ = SteamManager.Initialized;
            #endregion

            #region Data.Path Initialization
            _ = Data.Path;
            #endregion

            #region TimerUtil Initialization
            TimerUtil.InitializeInstance();
            #endregion

            #region Utilities Initialization
            Universe.DensityMapGenerationUtil.InitializeInstance();
            #endregion

            #region Scene Initialization
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

            #endregion

            IsInitializing = false;
            IsInitialized = true;
            
            Debug.Log($"[MainManager] Finished Initialization in Scene '{activeSceneName}'.");
        }
        #endregion

        #region Static Methods
        public static void CreateNewGame(string gameName)
        {
            if (games.Contains(gameName))
            {
                throw new Exception("Cannot create new Game, because another Game with the same Name already exists!");
            }

            LoadScene(SceneType.Game, () =>
            {
                GameManager gameManager = FindObjectOfType<GameManager>();
                gameManager.InitializeGame(gameName);
            });
        }

        public static void CreateNewGame(string gameName, Universe.GenerationSettings generationSettings)
        {
            if (games.Contains(gameName))
            {
                throw new Exception("Cannot create new Game, because another Game with the same Name already exists!");
            }

            LoadScene(SceneType.Game, () =>
            {
                GameManager gameManager = FindObjectOfType<GameManager>();
                gameManager.InitializeGame(gameName, generationSettings);
            });
        }

        public static void LoadGame(string gameName)
        {
            if (!games.Contains(gameName))
            {
                throw new Exception("Cannot load Game, because it does not exist!");
            }

            LoadScene(SceneType.Game, () =>
            {
                Game game = games.GetGame(gameName);
                GameManager gameManager = FindObjectOfType<GameManager>();
                gameManager.InitializeGame(game);
            });
        }

        public static void DeleteGame(string gameName)
        {
            if (GameManager.Initialized && GameManager.Instance.CurrentGame.Name == gameName)
            {
                throw new Exception("Cannot delete Game when it is loaded!");
            }

            Game game = games.GetGame(gameName);
            Game.DeleteGame(game);
        }

        public static void RenameGame(string oldGameName, string newGameName)
        {
            if (GameManager.Initialized && GameManager.Instance.CurrentGame.Name == oldGameName)
            {
                throw new Exception("Cannot rename Game when it is loaded!");
            }

            Game game = games.GetGame(oldGameName);
            Game.Rename(game, newGameName);
        }

        public static void LoadMainMenu()
        {
            LoadScene(SceneType.MainMenu);
        }

        private static void LoadScene(SceneType sceneType, Action postLoadAction = null)
        {
            string sceneName = Enum.GetName(typeof(SceneType), sceneType);
            Debug.Log($"[MainManager] Loading Scene '{sceneName}'.");
            switch (sceneType)
            {
                case SceneType.MainMenu:
                    if (SceneManager.GetActiveScene().name == "Game")
                    {
                        Game.Save(GameManager.Instance.CurrentGame);
                    }

                    GameManager.AddPostInitializationAction(postLoadAction);
                    Instance.StartCoroutine(Instance.LoadSceneAsynchronously(sceneName));
                    break;
                case SceneType.Game:
                    GameManager.AddPostInitializationAction(postLoadAction);
                    Instance.StartCoroutine(Instance.LoadSceneAsynchronously(sceneName));
                    break;
                default:
                    throw new ArgumentException($"Scene Type '{sceneName}' not supported!");
            }
            Debug.Log($"[MainManager] Finished loading Scene '{sceneName}'.");
        }

        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
        private static void PreInitialize()
        {
            string activeSceneName = SceneManager.GetActiveScene().name;
            Debug.Log($"[MainManager] Starting Pre-Initialization in Scene '{activeSceneName}'.");

            IsPreInitializing = true;

            #region Pre-Initialization
            Namespace rootNamespace = new Namespace("LooCast");
            
            Namespace aiNamespace = new Namespace("AI", rootNamespace);
            Type allyAI = new Type(typeof(AllyAI), aiNamespace);
            Type enemyAI = new Type(typeof(EnemyAI), aiNamespace);

            Namespace allyNamespace = new Namespace("Ally", rootNamespace);
            Type ally = new Type(typeof(Ally), allyNamespace);
            Type smolAlly = new Type(typeof(SmolAlly), allyNamespace);

            Namespace aoeNamespace = new Namespace("AOE", rootNamespace);

            Namespace arcNamespace = new Namespace("Arc", rootNamespace);
            
            Namespace asteroidNamespace = new Namespace("Asteroid", rootNamespace);

            Namespace attributeNamespace = new Namespace("Attribute", rootNamespace);
            Namespace attributeStatNamespace = new Namespace("Stat", attributeNamespace);

            Namespace backgroundNamespace = new Namespace("Background", rootNamespace);
            
            Namespace chanceNamespace = new Namespace("Chance", rootNamespace);
            
            Namespace coreNamespace = new Namespace("Core", rootNamespace);
            Namespace coreRegistryNamespace = new Namespace("Registry", coreNamespace);
            Type namespaceRegistryType = new Type(typeof(Registry<NamespaceIdentifier, Namespace>), coreRegistryNamespace);
            Type typeRegistryType = new Type(typeof(Registry<TypeIdentifier, Type>), coreRegistryNamespace);
            Type instanceRegistryType = new Type(typeof(Registry<InstanceIdentifier, Instance>), coreRegistryNamespace);
            
            Namespace currencyNamespace = new Namespace("Currency", rootNamespace);
            
            Namespace dataNamespace = new Namespace("Data", rootNamespace);
            
            Namespace diagnosticNamespace = new Namespace("Diagnostic", rootNamespace);

            Namespace enemyNamespace = new Namespace("Enemy", rootNamespace);
            
            Namespace eventNamespace = new Namespace("Event", rootNamespace);
            
            Namespace experienceNamespace = new Namespace("Experience", rootNamespace);

            Namespace gameNamespace = new Namespace("Game", rootNamespace);

            Namespace generatorNamespace = new Namespace("Generator", rootNamespace);
            
            Namespace healthNamespace = new Namespace("Health", rootNamespace);

            Namespace identifierNamespace = new Namespace("Identifier", rootNamespace);

            Namespace indicatorNamespace = new Namespace("Indicator", rootNamespace);

            Namespace inventoryNamespace = new Namespace("Inventory", rootNamespace);

            Namespace itemNamespace = new Namespace("Item", rootNamespace);
            
            Namespace mainMenuNamespace = new Namespace("MainMenu", rootNamespace);
            
            Namespace mathNamespace = new Namespace("Math", rootNamespace);
            Namespace mathMapNamespace = new Namespace("Map", mathNamespace);
            
            Namespace missionNamespace = new Namespace("Mission", rootNamespace);
            Namespace missionRewardNamespace = new Namespace("Reward", missionNamespace);
            Namespace missionTargetNamespace = new Namespace("Target", missionNamespace);
            Namespace missionTaskNamespace = new Namespace("Task", missionNamespace);
            Namespace missionTriggerNamespace = new Namespace("Trigger", missionNamespace);
            
            Namespace movementNamespace = new Namespace("Movement", rootNamespace);
            Namespace movementEffectNamespace = new Namespace("Effect", movementNamespace);

            Namespace noiseNamespace = new Namespace("Noise", rootNamespace);
            
            Namespace observerNamespace = new Namespace("Observer", rootNamespace);
            
            Namespace orbNamespace = new Namespace("Orb", rootNamespace);
            
            Namespace particleNamespace = new Namespace("Particle", rootNamespace);

            Namespace playerNamespace = new Namespace("Player", rootNamespace);

            Namespace projectileNamespace = new Namespace("Projectile", rootNamespace);
            
            Namespace randomNamespace = new Namespace("Random", rootNamespace);

            Namespace resourceNamespace = new Namespace("Resource", rootNamespace);

            Namespace soundNamespace = new Namespace("Sound", rootNamespace);

            Namespace spawnerNamespace = new Namespace("Spawner", rootNamespace);
            
            Namespace stateMachineNamespace = new Namespace("StateMachine", rootNamespace);
            
            Namespace stationNamespace = new Namespace("Station", rootNamespace);
            
            Namespace statisticNamespace = new Namespace("Statistic", rootNamespace);
            
            Namespace steamworksNamespace = new Namespace("Steamworks", rootNamespace);

            Namespace targetNamespace = new Namespace("Target", rootNamespace);
            
            Namespace testNamespace = new Namespace("Test", rootNamespace);
            
            Namespace uiNamespace = new Namespace("UI", rootNamespace);
            Namespace uiBarNamespace = new Namespace("Bar", uiNamespace);
            Namespace uiButtonNamespace = new Namespace("Button", uiNamespace);
            Namespace uiCanvasNamespace = new Namespace("Canvas", uiNamespace);
            Namespace uiCursorNamespace = new Namespace("Cursor", uiNamespace);
            Namespace uiHudNamespace = new Namespace("HUD", uiNamespace);
            Namespace uiInspectorNamespace = new Namespace("Inspector", uiNamespace);
            Namespace uiInventoryNamespace = new Namespace("Inventory", uiNamespace);
            Namespace uiLevelNamespace = new Namespace("Level", uiNamespace);
            Namespace uiPanelNamespace = new Namespace("Panel", uiNamespace);
            Namespace uiRewardNamespace = new Namespace("Reward", uiNamespace);
            Namespace uiScreenNamespace = new Namespace("Screen", uiNamespace);
            Namespace uiSliderNamespace = new Namespace("Slider", uiNamespace);
            Namespace uiTabNamespace = new Namespace("Tab", uiNamespace);
            Namespace uiTaskNamespace = new Namespace("Task", uiNamespace);
            Namespace uiTimerNamespace = new Namespace("Timer", uiNamespace);
            Namespace uiTitleNamespace = new Namespace("Title", uiNamespace);
            Namespace uiValueNamespace = new Namespace("Value", uiNamespace);
            
            Namespace universeNamespace = new Namespace("Universe", rootNamespace);
            
            Namespace utilNamespace = new Namespace("Util", rootNamespace);
            Namespace utilCollectionsNamespace = new Namespace("Collections", utilNamespace);
            
            Namespace variableNamespace = new Namespace("Variable", rootNamespace);
            

            RegistryManager.Instance.Initialize();
            NamespaceManager.Instance.Initialize();
            TypeManager.Instance.Initialize();
            InstanceManager.Instance.Initialize();

            // Register all Namespaces, Types and Instances
            #endregion

            IsPreInitializing = false;
            IsPreInitialized = true;

            Debug.Log($"[MainManager] Finished Pre-Initialization in Scene '{activeSceneName}'.");
            _ = Instance;
        }

        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.AfterSceneLoad)]
        private static void PostInitialize()
        {
            string activeSceneName = SceneManager.GetActiveScene().name;
            Debug.Log($"[MainManager] Starting Post-Initialization in Scene '{activeSceneName}'.");

            IsPostInitializing = true;

            #region Post-Initialization

            #endregion

            IsPostInitializing = false;
            IsPostInitialized = true;

            Debug.Log($"[MainManager] Finished Post-Initialization in Scene '{activeSceneName}'.");
        }
        #endregion

        #region Methods
        #endregion

        #region Coroutines
        public IEnumerator LoadSceneAsynchronously(string sceneIndex)
        {
            LoadingScreen loadingScreen = FindObjectOfType<LoadingScreen>();
            yield return loadingScreen.LoadSceneAsynchronously(sceneIndex);
        }
        #endregion
    }
}
