using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace LooCast
{
    using LooCast.Game;

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
        public Instance LooCastInstance => looCastInstance;
        #endregion

        #region Fields
        private Namespace looCastNamespace;
        private Type looCastType;
        private Instance looCastInstance;
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
            string activeSceneName = SceneManager.GetActiveScene().name;
            
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
            InstanceManager instanceManager = InstanceManager.Instance;
            
            looCastNamespace = namespaceManager.GetNamespace("LooCast");
            looCastType = new Type(typeof(MainManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);
            
            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

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

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.EarlyPreInitialize();
            }
            #endregion

            #region DEPRECATED! [TO BE MOVED!]
            Namespace missionNamespace = new Namespace("Mission", rootNamespace);
            Namespace missionRewardNamespace = new Namespace("Reward", missionNamespace);
            Namespace missionTargetNamespace = new Namespace("Target", missionNamespace);
            Namespace missionTaskNamespace = new Namespace("Task", missionNamespace);
            Namespace missionTriggerNamespace = new Namespace("Trigger", missionNamespace);
            Type conquerStationMissionType = new Type(typeof(ConquerStationMission), missionNamespace);
            Type missionType = new Type(typeof(Mission.Mission), missionNamespace);
            Type missionManagerType = new Type(typeof(MissionManager), missionNamespace);
            Type missionProviderType = new Type(typeof(MissionProvider), missionNamespace);
            Type missionRarityType = new Type(typeof(MissionRarity), missionNamespace);
            Type missionStateType = new Type(typeof(MissionState), missionNamespace);
            Type creditsMissionRewardType = new Type(typeof(Mission.Reward.CreditsMissionReward), missionRewardNamespace);
            Type itemMissionRewardType = new Type(typeof(Mission.Reward.ItemMissionReward), missionRewardNamespace);
            Type missionRewardType = new Type(typeof(Mission.Reward.MissionReward), missionRewardNamespace);
            Type reputationMissionRewardType = new Type(typeof(Mission.Reward.ReputationMissionReward), missionRewardNamespace);
            Type missionTargetType = new Type(typeof(MissionTarget), missionTargetNamespace);
            Type iMissionTaskLockStateType = new Type(typeof(IMissionTaskLockState), missionTaskNamespace);
            Type lockedMissionTaskLockStateType = new Type(typeof(LockedMissionTaskLockState), missionTaskNamespace);
            Type missionTaskType = new Type(typeof(Mission.Task.MissionTask), missionTaskNamespace);
            Type unlockedMissionTaskLockStateType = new Type(typeof(UnlockedMissionTaskLockState), missionTaskNamespace);
            Type missionTriggerType = new Type(typeof(MissionTrigger), missionTriggerNamespace);

            Namespace randomNamespace = new Namespace("Random", rootNamespace);
            Type randomType = new Type(typeof(Random.Random), randomNamespace);
            Type seededRandomType = new Type(typeof(SeededRandom), randomNamespace);

            Namespace resourceNamespace = new Namespace("Resource", rootNamespace);
            Type resourceType = new Type(typeof(Resource.Resource), resourceNamespace);

            Namespace soundNamespace = new Namespace("Sound", rootNamespace);
            Type soundType = new Type(typeof(Sound.Sound), soundNamespace);
            Type soundHandlerType = new Type(typeof(SoundHandler), soundNamespace);
            Type gameSoundHandlerType = new Type(typeof(GameSoundHandler), soundNamespace);
            Type menuSoundHandlerType = new Type(typeof(MenuSoundHandler), soundNamespace);

            Namespace spawnerNamespace = new Namespace("Spawner", rootNamespace);
            Type spawnerType = new Type(typeof(Spawner.Spawner), spawnerNamespace);
            Type allySpawnerType = new Type(typeof(AllySpawner), spawnerNamespace);
            Type enemySpawnerType = new Type(typeof(EnemySpawner), spawnerNamespace);

            Namespace stateMachineNamespace = new Namespace("StateMachine", rootNamespace);
            Type finiteStateMachineType = new Type(typeof(FiniteStateMachine<object>), stateMachineNamespace);
            Type stateType = new Type(typeof(State<object>), stateMachineNamespace);

            Namespace stationNamespace = new Namespace("Station", rootNamespace);
            Type stationType = new Type(typeof(Station.Station), stationNamespace);
            Type allyStationType = new Type(typeof(AllyStation), stationNamespace);
            Type enemyStationType = new Type(typeof(EnemyStation), stationNamespace);

            Namespace statisticNamespace = new Namespace("Statistic", rootNamespace);
            Type killsStatisticType = new Type(typeof(KillsStatistic), statisticNamespace);

            Namespace steamworksNamespace = new Namespace("Steamworks", rootNamespace);
            Type steamManagerType = new Type(typeof(SteamManager), steamworksNamespace);

            Namespace targetNamespace = new Namespace("Target", rootNamespace);
            Type targetType = new Type(typeof(Target.Target), targetNamespace);

            Namespace testNamespace = new Namespace("Test", rootNamespace);
            Type mapDisplayType = new Type(typeof(MapDisplay), testNamespace);
            Type perlinMapGeneratorType = new Type(typeof(PerlinMapGenerator), testNamespace);
            Type perlinMapGeneratorGPUType = new Type(typeof(PerlinMapGeneratorGPU), testNamespace);
            Type voronoiMapGeneratorType = new Type(typeof(VoronoiMapGenerator), testNamespace);
            Type voronoiMapGeneratorGPUType = new Type(typeof(VoronoiMapGeneratorGPU), testNamespace);

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
            Type uiVersionInfoType = new Type(typeof(VersionInfo), uiNamespace);
            Type uiBarType = new Type(typeof(Bar), uiBarNamespace);
            Type uiEnergyBarType = new Type(typeof(EnergyBar), uiBarNamespace);
            Type uiExperienceBarType = new Type(typeof(ExperienceBar), uiBarNamespace);
            Type uiHealthBarType = new Type(typeof(HealthBar), uiBarNamespace);
            Type uiButtonType = new Type(typeof(Button), uiButtonNamespace);
            Type uiAttributeSetButtonType = new Type(typeof(AttributeSetButton), uiButtonNamespace);
            Type uiCreateGameButtonType = new Type(typeof(CreateGameButton), uiButtonNamespace);
            Type uiHardresetButtonType = new Type(typeof(HardresetButton), uiButtonNamespace);
            Type uiLoadGameButtonType = new Type(typeof(LoadGameButton), uiButtonNamespace);
            Type uiLoadMainMenuButtonType = new Type(typeof(LoadMainMenuButton), uiButtonNamespace);
            Type uiMissionButtonType = new Type(typeof(MissionButton), uiButtonNamespace);
            Type uiStatSetButtonType = new Type(typeof(StatSetButton), uiButtonNamespace);
            Type uiTabButtonType = new Type(typeof(TabButton), uiButtonNamespace);
            Type uiCanvasType = new Type(typeof(Canvas), uiCanvasNamespace);
            Type uiGameCanvasType = new Type(typeof(GameCanvas), uiCanvasNamespace);
            Type uiInterfaceCanvasType = new Type(typeof(InterfaceCanvas), uiCanvasNamespace);
            Type uiMainMenuCanvasType = new Type(typeof(MainMenuCanvas), uiCanvasNamespace);
            Type uiScreenSpaceCameraCanvasType = new Type(typeof(ScreenSpaceCameraCanvas), uiCanvasNamespace);
            Type uiScreenSpaceOverlayCanvasType = new Type(typeof(ScreenSpaceOverlayCanvas), uiCanvasNamespace);
            Type uiWorldSpaceCanvasType = new Type(typeof(WorldSpaceCanvas), uiCanvasNamespace);
            Type uiAsteroidCursorType = new Type(typeof(AsteroidCursor), uiCursorNamespace);
            Type uiMissionButtonCursorType = new Type(typeof(MissionButtonCursor), uiCursorNamespace);
            Type uiHUDType = new Type(typeof(HUD), uiHudNamespace);
            Type uiActiveMissionInspectorType = new Type(typeof(ActiveMissionInspector), uiInspectorNamespace);
            Type uiAsteroidInspectorType = new Type(typeof(AsteroidInspector), uiInspectorNamespace);
            Type uiInventoryHotbarType = new Type(typeof(InventoryHotbar), uiInventoryNamespace);
            Type uiInventoryItemType = new Type(typeof(InventoryItem), uiInventoryNamespace);
            Type uiInventorySlotType = new Type(typeof(InventorySlot), uiInventoryNamespace);
            Type uiInventorySlotCursorType = new Type(typeof(InventorySlotCursor), uiInventoryNamespace);
            Type uiLevelType = new Type(typeof(Level), uiLevelNamespace);
            Type uiAttributeLevelType = new Type(typeof(AttributeLevel), uiLevelNamespace);
            Type uiStatLeveltype = new Type(typeof(StatLevel), uiLevelNamespace);
            Type uiPanelType = new Type(typeof(Panel), uiPanelNamespace);
            Type uiAsteroidInfoPanelType = new Type(typeof(AsteroidInfoPanel), uiPanelNamespace);
            Type uiAsteroidResourceDeposistsPanelType = new Type(typeof(AsteroidResourceDepositsPanel), uiPanelNamespace);
            Type uiStationBlackmarketPanelType = new Type(typeof(StationBlackmarketPanel), uiPanelNamespace);
            Type uiStationHUBPanelType = new Type(typeof(StationHUBPanel), uiPanelNamespace);
            Type uiStationManufacturingPanelType = new Type(typeof(StationManufacturingPanel), uiPanelNamespace);
            Type uiStationMarketPanelType = new Type(typeof(StationMarketPanel), uiPanelNamespace);
            Type uiStationMissionPanelType = new Type(typeof(StationMissionPanel), uiPanelNamespace);
            Type uiStationUpgradesPanelType = new Type(typeof(StationUpgradesPanel), uiPanelNamespace);
            Type uiMissionRewardType = new Type(typeof(UI.Reward.MissionReward), uiRewardNamespace);
            Type uiCreditsMissionRewardType = new Type(typeof(UI.Reward.CreditsMissionReward), uiRewardNamespace);
            Type uiItemMissionRewardType = new Type(typeof(UI.Reward.ItemMissionReward), uiRewardNamespace);
            Type uiReputationMissionRewardType = new Type(typeof(UI.Reward.ReputationMissionReward), uiRewardNamespace);
            Type uiScreenType = new Type(typeof(Screen), uiScreenNamespace);
            Type uiDeathScreenType = new Type(typeof(DeathScreen), uiScreenNamespace);
            Type uiLoadGameScreenType = new Type(typeof(LoadGameScreen), uiScreenNamespace);
            Type uiLoadingScreenType = new Type(typeof(LoadingScreen), uiScreenNamespace);
            Type uiMainScreenType = new Type(typeof(MainScreen), uiScreenNamespace);
            Type uiNewGameScreenType = new Type(typeof(NewGameScreen), uiScreenNamespace);
            Type uiPauseScreenType = new Type(typeof(PauseScreen), uiScreenNamespace);
            Type uiSettingsScreenType = new Type(typeof(SettingsScreen), uiScreenNamespace);
            Type uiStationScreenType = new Type(typeof(StationScreen), uiScreenNamespace);
            Type uiStatsScreenType = new Type(typeof(StatsScreen), uiScreenNamespace);
            Type uiSliderType = new Type(typeof(Slider), uiSliderNamespace);
            Type uiDifficultySliderType = new Type(typeof(DifficultySlider), uiSliderNamespace);
            Type uiVolumeSliderType = new Type(typeof(VolumeSlider), uiSliderNamespace);
            Type uiEffectVolumeSliderType = new Type(typeof(EffectVolumeSlider), uiSliderNamespace);
            Type uiMasterVolumeSliderType = new Type(typeof(MasterVolumeSlider), uiSliderNamespace);
            Type uiMusicVolumeSliderType = new Type(typeof(MusicVolumeSlider), uiSliderNamespace);
            Type uiUIVolumeSliderType = new Type(typeof(UIVolumeSlider), uiSliderNamespace);
            Type uiTabType = new Type(typeof(Tab), uiTabNamespace);
            Type uiTabGroupType = new Type(typeof(TabGroup), uiTabNamespace);
            Type uiMissionTaskType = new Type(typeof(UI.Task.MissionTask), uiTaskNamespace);
            Type uiMissionTaskContainerType = new Type(typeof(UI.Task.MissionTaskContainer), uiTaskNamespace);
            Type uiRoundTimerType = new Type(typeof(RoundTimer), uiTimerNamespace);
            Type uiTitleType = new Type(typeof(Title), uiTitleNamespace);
            Type uiValueType = new Type(typeof(Value), uiValueNamespace);
            Type uiCoinsValueType = new Type(typeof(CoinsValue), uiValueNamespace);
            Type uiExperienceLevelValueType = new Type(typeof(ExperienceLevelValue), uiValueNamespace);
            Type uiStatValueType = new Type(typeof(StatValue), uiValueNamespace);
            Type uiTokensValueType = new Type(typeof(TokensValue), uiValueNamespace);

            Namespace universeNamespace = new Namespace("Universe", rootNamespace);
            Type universeType = new Type(typeof(Universe.Universe), universeNamespace);
            
            
            #endregion

            #endregion

            IsEarlyPreInitializing = false;
            IsEarlyPreInitialized = true;
            Debug.Log($"[MainManager] Finished Early Pre-Initialization in Scene '{activeSceneName}'.");

            PreInitialize();
        }

        private void PreInitialize()
        {
            string activeSceneName = SceneManager.GetActiveScene().name;
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
            string activeSceneName = SceneManager.GetActiveScene().name;
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
            string activeSceneName = SceneManager.GetActiveScene().name;
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
            string activeSceneName = SceneManager.GetActiveScene().name;
            IsInitializing = true;
            Debug.Log($"[MainManager] Starting Initialization in Scene '{activeSceneName}'.");

            #region Initialization

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

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.Initialize();
            }
            #endregion

            #region DEPRECATED! [TO BE MOVED!]

            #region SteamManager
            _ = SteamManager.Initialized;
            #endregion

            #region Data.Path
            _ = Data.Path;
            #endregion

            #region TimerUtil
            TimerUtil.InitializeInstance();
            #endregion

            #region Utilities
            Universe.DensityMapGenerationUtil.InitializeInstance();
            #endregion

            #region Scene
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

            #endregion

            IsInitializing = false;
            IsInitialized = true;
            Debug.Log($"[MainManager] Finished Initialization in Scene '{activeSceneName}'.");

            LateInitialize();
        }

        private void LateInitialize()
        {
            string activeSceneName = SceneManager.GetActiveScene().name;
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
            string activeSceneName = SceneManager.GetActiveScene().name;
            IsEarlyPostInitializing = true;
            Debug.Log($"[MainManager] Starting Early Post-Initialization in Scene '{activeSceneName}'.");

            #region Early Post-Initialization

            #region MainManager

            #endregion

            #region Core Module Managers
            LooCast.Core.CoreManager looCastCoreManager = CoreManager.Instance;
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
            string activeSceneName = SceneManager.GetActiveScene().name;
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
            string activeSceneName = SceneManager.GetActiveScene().name;
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
            string activeSceneName = SceneManager.GetActiveScene().name;
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
            string activeSceneName = SceneManager.GetActiveScene().name;
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
            string activeSceneName = SceneManager.GetActiveScene().name;
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
            string activeSceneName = SceneManager.GetActiveScene().name;
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
            string activeSceneName = SceneManager.GetActiveScene().name;
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
            string activeSceneName = SceneManager.GetActiveScene().name;
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
            string activeSceneName = SceneManager.GetActiveScene().name;
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
            string activeSceneName = SceneManager.GetActiveScene().name;
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
            string activeSceneName = SceneManager.GetActiveScene().name;
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
