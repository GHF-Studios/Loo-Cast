using System;
using System.IO;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace LooCast.Core
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

    public class MainManager : Component
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
        private static MainManager instance;
        private static Games games;
        private static Game gameToBeLoaded;
        public static float saveInterval = 30.0f;
        #endregion

        #region Unity Callbacks
        private void Awake()
        {
            Debug.Log($"[MainManager] Starting Initialization.");

            #region Initialization

            IsInitializing = true;

            #region Data.Path
            _ = Data.Path;
            #endregion

            #region MainManager
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
            
            Debug.Log($"[MainManager] Finished Initialization.");
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
                    throw new ArgumentException($"[MainManager] Scene Type '{sceneName}' not supported!");
            }
            Debug.Log($"[MainManager] Finished loading Scene '{sceneName}'.");
        }

        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.BeforeSceneLoad)]
        private static void PreInitialize()
        {
            Debug.Log($"[MainManager] Starting Pre-Initialization.");

            #region Pre-Initialization
            
            IsPreInitializing = true;

            #region IdentifierManager
            IdentifierManager idManager = IdentifierManager.Instance;
            idManager.Initialize();

            #region Namespaces
            // Root Namespace
            IdentifierManager.NamespaceIdentifier rootNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateRootNamespace("LooCast");
            
            // Top Level Namespaces
            IdentifierManager.NamespaceIdentifier aiNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "AI");
            
            IdentifierManager.NamespaceIdentifier allyNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Ally");
            IdentifierManager.NamespaceIdentifier allyDataNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(allyNamespace, "Data");
            IdentifierManager.NamespaceIdentifier allyDataRuntimeNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(allyDataNamespace, "Runtime");
            
            IdentifierManager.NamespaceIdentifier aoeNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "AOE");
            
            IdentifierManager.NamespaceIdentifier apiNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "API");
            
            IdentifierManager.NamespaceIdentifier arcNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Arc");
            
            IdentifierManager.NamespaceIdentifier asteroidNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Asteroid");
            IdentifierManager.NamespaceIdentifier asteroidDataNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(asteroidNamespace, "Data");
            IdentifierManager.NamespaceIdentifier asteroidDataRuntimeNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(asteroidDataNamespace, "Runtime");

            IdentifierManager.NamespaceIdentifier attributeNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Attribute");
            IdentifierManager.NamespaceIdentifier attributeStatNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(attributeNamespace, "Stat");
            
            IdentifierManager.NamespaceIdentifier backgroundNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Background");
            
            IdentifierManager.NamespaceIdentifier chanceNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Chance");
            
            IdentifierManager.NamespaceIdentifier coreNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Core");
            
            IdentifierManager.NamespaceIdentifier currencyNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Currency");
            
            IdentifierManager.NamespaceIdentifier dataNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Data");
            IdentifierManager.NamespaceIdentifier dataRuntimeNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(dataNamespace, "Runtime");
            
            IdentifierManager.NamespaceIdentifier diagnosticNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Diagnostic");
            
            IdentifierManager.NamespaceIdentifier enemyNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Enemy");
            IdentifierManager.NamespaceIdentifier enemyDataNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(enemyNamespace, "Data");
            IdentifierManager.NamespaceIdentifier enemyDataRuntimeNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(enemyDataNamespace, "Runtime");
            
            IdentifierManager.NamespaceIdentifier eventNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Event");
            
            IdentifierManager.NamespaceIdentifier experienceNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Experience");
            IdentifierManager.NamespaceIdentifier experienceDataNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(experienceNamespace, "Data");
            IdentifierManager.NamespaceIdentifier experienceDataRuntimeNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(experienceDataNamespace, "Runtime");
            
            IdentifierManager.NamespaceIdentifier gameNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Game");
            
            IdentifierManager.NamespaceIdentifier generatorNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Generator");
            IdentifierManager.NamespaceIdentifier generatorDataNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(generatorNamespace, "Data");

            IdentifierManager.NamespaceIdentifier healthNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Health");
            IdentifierManager.NamespaceIdentifier healthDataNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(healthNamespace, "Data");
            IdentifierManager.NamespaceIdentifier healthDataRuntimeNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(healthDataNamespace, "Runtime");

            IdentifierManager.NamespaceIdentifier identifierNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Identifier");
            
            IdentifierManager.NamespaceIdentifier indicatorNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Indicator");
            
            IdentifierManager.NamespaceIdentifier inventoryNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Inventory");
            IdentifierManager.NamespaceIdentifier inventoryDataNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(inventoryNamespace, "Data");
            IdentifierManager.NamespaceIdentifier inventoryDataRuntimeNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(inventoryDataNamespace, "Runtime");

            IdentifierManager.NamespaceIdentifier itemNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Item");
            IdentifierManager.NamespaceIdentifier itemDataNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(itemNamespace, "Data");

            IdentifierManager.NamespaceIdentifier mainMenuNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "MainMenu");
            
            IdentifierManager.NamespaceIdentifier mathNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Math");
            IdentifierManager.NamespaceIdentifier mathGeometryNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(mathNamespace, "Geometry");
            IdentifierManager.NamespaceIdentifier mathMapNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(mathNamespace, "Map");
            
            IdentifierManager.NamespaceIdentifier missionNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Mission");
            IdentifierManager.NamespaceIdentifier missionDataNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(missionNamespace, "Data");
            IdentifierManager.NamespaceIdentifier missionRewardNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(missionNamespace, "Reward");
            IdentifierManager.NamespaceIdentifier missionTargetNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(missionNamespace, "Target");
            IdentifierManager.NamespaceIdentifier missionTaskNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(missionNamespace, "Task");
            IdentifierManager.NamespaceIdentifier missionTriggerNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(missionNamespace, "Trigger");
            
            IdentifierManager.NamespaceIdentifier modNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Mod");
            
            IdentifierManager.NamespaceIdentifier moduleNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Module");
            
            IdentifierManager.NamespaceIdentifier movementNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Movement");
            IdentifierManager.NamespaceIdentifier movementDataNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(movementNamespace, "Data");
            IdentifierManager.NamespaceIdentifier movementDataRuntimeNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(movementDataNamespace, "Runtime");
            IdentifierManager.NamespaceIdentifier movementEffectNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(movementNamespace, "Effect");

            IdentifierManager.NamespaceIdentifier noiseNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Noise");
            
            IdentifierManager.NamespaceIdentifier observerNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Observer");
            IdentifierManager.NamespaceIdentifier observerDataNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(observerNamespace, "Data");
            
            IdentifierManager.NamespaceIdentifier orbNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Orb");
            
            IdentifierManager.NamespaceIdentifier particleNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Particle");
            
            IdentifierManager.NamespaceIdentifier playerNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Player");
            IdentifierManager.NamespaceIdentifier playerDataNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(playerNamespace, "Data");
            IdentifierManager.NamespaceIdentifier playerDataRuntimeNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(playerDataNamespace, "Runtime");
            
            IdentifierManager.NamespaceIdentifier projectileNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Projectile");
            
            IdentifierManager.NamespaceIdentifier randomNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Random");
            
            IdentifierManager.NamespaceIdentifier registryNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Registry");
            
            IdentifierManager.NamespaceIdentifier resourceNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Resource");
            
            IdentifierManager.NamespaceIdentifier soundNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Sound");

            IdentifierManager.NamespaceIdentifier spawnerNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Spawner");
            IdentifierManager.NamespaceIdentifier spawnerDataNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(spawnerNamespace, "Data");
            
            IdentifierManager.NamespaceIdentifier stateMachineNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "StateMachine");
            
            IdentifierManager.NamespaceIdentifier stationNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Station");
            IdentifierManager.NamespaceIdentifier stationDataNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(stationNamespace, "Data");
            IdentifierManager.NamespaceIdentifier stationDataRuntimeNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(stationDataNamespace, "Runtime");
            
            IdentifierManager.NamespaceIdentifier statisticNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Statistic");
            
            IdentifierManager.NamespaceIdentifier steamworksNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Steamworks");

            IdentifierManager.NamespaceIdentifier targetNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Target");

            IdentifierManager.NamespaceIdentifier testNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Test");

            IdentifierManager.NamespaceIdentifier transformNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Transform");

            IdentifierManager.NamespaceIdentifier uiNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "UI");
            IdentifierManager.NamespaceIdentifier uiAnimationNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(uiNamespace, "Animation");
            IdentifierManager.NamespaceIdentifier uiBarNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(uiNamespace, "Bar");
            IdentifierManager.NamespaceIdentifier uiButtonNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(uiNamespace, "Button");
            IdentifierManager.NamespaceIdentifier uiCanvasNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(uiNamespace, "Canvas");
            IdentifierManager.NamespaceIdentifier uiCursorNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(uiNamespace, "Cursor");
            IdentifierManager.NamespaceIdentifier uiHUDNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(uiNamespace, "HUD");
            IdentifierManager.NamespaceIdentifier uiInspectorNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(uiNamespace, "Inspector");
            IdentifierManager.NamespaceIdentifier uiInventoryNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(uiNamespace, "Inventory");
            IdentifierManager.NamespaceIdentifier uiLevelNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(uiNamespace, "Level");
            IdentifierManager.NamespaceIdentifier uiOverlayNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(uiNamespace, "Overlay");
            IdentifierManager.NamespaceIdentifier uiPanelNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(uiNamespace, "Panel");
            IdentifierManager.NamespaceIdentifier uiRewardNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(uiNamespace, "Reward");
            IdentifierManager.NamespaceIdentifier uiScreenNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(uiNamespace, "Screen");
            IdentifierManager.NamespaceIdentifier uiSliderNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(uiNamespace, "Slider");
            IdentifierManager.NamespaceIdentifier uiTabNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(uiNamespace, "Tab");
            IdentifierManager.NamespaceIdentifier uiTaskNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(uiNamespace, "Task");
            IdentifierManager.NamespaceIdentifier uiTimerNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(uiNamespace, "Timer");
            IdentifierManager.NamespaceIdentifier uiTitleNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(uiNamespace, "Title");
            IdentifierManager.NamespaceIdentifier uiTooltipNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(uiNamespace, "Tooltip");
            IdentifierManager.NamespaceIdentifier uiValueNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(uiNamespace, "Value");
            
            IdentifierManager.NamespaceIdentifier universeNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Universe");
            
            IdentifierManager.NamespaceIdentifier utilNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Util");
            IdentifierManager.NamespaceIdentifier utilCollectionsNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(utilNamespace, "Collections");
            IdentifierManager.NamespaceIdentifier utilCollectionsConcurrentNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(utilCollectionsNamespace, "Concurrent");
            IdentifierManager.NamespaceIdentifier utilCollectionsGenericNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(utilCollectionsNamespace, "Generic");
            
            IdentifierManager.NamespaceIdentifier variableNamespace = (IdentifierManager.NamespaceIdentifier)idManager.CreateNamespace(rootNamespace, "Variable");
            #endregion

            #region Types
            IdentifierManager.ComponentTypeIdentifier<AI.AllyAI> allyAIComponentType = (IdentifierManager.ComponentTypeIdentifier<AI.AllyAI>)idManager.CreateRootGameObjectType(aiNamespace, typeof(AI.AllyAI));
            IdentifierManager.ComponentTypeIdentifier<AI.EnemyAI> enemyAIComponentType = (IdentifierManager.ComponentTypeIdentifier<AI.EnemyAI>)idManager.CreateRootGameObjectType(aiNamespace, typeof(AI.EnemyAI));
            
            IdentifierManager.GameObjectTypeIdentifier<Ally.Ally> allyGameObjectType = (IdentifierManager.GameObjectTypeIdentifier<Ally.Ally>)idManager.CreateRootGameObjectType(allyNamespace, typeof(Ally.Ally));
            IdentifierManager.GameObjectDataTypeIdentifier<Ally.Data.AllyData> allyGameObjectDataType = (IdentifierManager.GameObjectDataTypeIdentifier<Ally.Data.AllyData>)idManager.CreateGameObjectDataType(allyGameObjectType, typeof(Ally.Data.AllyData));
            IdentifierManager.GameObjectRuntimeDataTypeIdentifier<Ally.Data.Runtime.AllyRuntimeSet> allyGameObjectRuntimeDataType = (IdentifierManager.GameObjectRuntimeDataTypeIdentifier<Ally.Data.Runtime.AllyRuntimeSet>)idManager.CreateGameObjectRuntimeDataType(allyGameObjectType, typeof(Ally.Data.Runtime.AllyRuntimeSet));

            IdentifierManager.GameObjectTypeIdentifier<Ally.SmolAlly> smolAllyGameObjectType = (IdentifierManager.GameObjectTypeIdentifier<Ally.SmolAlly>)idManager.CreateGameObjectType(allyGameObjectType, typeof(Ally.SmolAlly));
            
            IdentifierManager.GameObjectTypeIdentifier<AOE.FreezeZone> freezeZoneGameObjectType = (IdentifierManager.GameObjectTypeIdentifier<AOE.FreezeZone>)idManager.CreateGameObjectType(allyGameObjectType, typeof(AOE.FreezeZone));

            IdentifierManager.GameObjectTypeIdentifier<Arc.Arc> arcGameObjectType = (IdentifierManager.GameObjectTypeIdentifier<Arc.Arc>)idManager.CreateRootGameObjectType(arcNamespace, typeof(Arc.Arc));
            
            idManager.CreateRootGameObjectType(asteroidNamespace, typeof(Asteroid.Asteroid));
            
            idManager.CreateRootGameObjectType(attributeNamespace, typeof(Attribute.Attribute));
            idManager.CreateRootGameObjectType(attributeStatNamespace, typeof(Attribute.Stat.Stat));
            
            idManager.CreateRootGameObjectType(backgroundNamespace, typeof(Background.Background));
            
            idManager.CreateRootGameObjectType(chanceNamespace, typeof(Chance.Chance));

            IdentifierManager.GameObjectTypeIdentifier<Enemy.Enemy> enemyObjectType = (IdentifierManager.GameObjectTypeIdentifier<Enemy.Enemy>)idManager.CreateRootGameObjectType(enemyNamespace, typeof(Enemy.Enemy));
            IdentifierManager.GameObjectTypeIdentifier<Enemy.SmolEnemy> smolEnemyObjectType = (IdentifierManager.GameObjectTypeIdentifier<Enemy.SmolEnemy>)idManager.CreateGameObjectType(enemyObjectType, typeof(Enemy.SmolEnemy));
            idManager.CreateRootGameObjectType(arcNamespace, typeof());
            idManager.CreateRootGameObjectType(arcNamespace, typeof());
            idManager.CreateRootGameObjectType(arcNamespace, typeof());
            idManager.CreateRootGameObjectType(arcNamespace, typeof());
            #endregion

            #region Singleton Instances

            #endregion

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
                Debug.LogError($"[MainManager] An error occured while initializing the ModManager!");
                Debug.LogException(exception);
                Debug.Log("[MainManager] Quitting Game.");
                Application.Quit();
            }

            try
            {
                ModuleManager.Instance.Initialize();
            }
            catch (Exception exception)
            {
                Debug.LogError($"[MainManager] An error occured while initializing the ModuleManager!");
                Debug.LogException(exception);
                Debug.Log("[MainManager] Quitting Game.");
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
            
            Debug.Log($"[MainManager] Finished Pre-Initialization.");
            _ = Instance;
        }

        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.AfterSceneLoad)]
        private static void PostInitialize()
        {
            Debug.Log($"[MainManager] Starting Post-Initialization.");

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

            Debug.Log($"[MainManager] Finished Post-Initialization.");
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
