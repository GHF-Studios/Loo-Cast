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
    using Util.Collections.Concurrent;
    using Util.Collections.Generic;
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
            Type allyAIType = new Type(typeof(AllyAI), aiNamespace);
            Type enemyAIType = new Type(typeof(EnemyAI), aiNamespace);

            Namespace allyNamespace = new Namespace("Ally", rootNamespace);
            Type allyType = new Type(typeof(Ally), allyNamespace);
            Type smolAllyType = new Type(typeof(SmolAlly), allyNamespace);

            Namespace aoeNamespace = new Namespace("AOE", rootNamespace);
            Type freezeZoneType = new Type(typeof(FreezeZone), aoeNamespace);

            Namespace arcNamespace = new Namespace("Arc", rootNamespace);
            Type arcType = new Type(typeof(Arc), arcNamespace);
            Type arcSegmentType = new Type(typeof(ArcSegment), arcNamespace);

            Namespace asteroidNamespace = new Namespace("Asteroid", rootNamespace);
            Type asteroidType = new Type(typeof(Asteroid), asteroidNamespace);

            Namespace attributeNamespace = new Namespace("Attribute", rootNamespace);
            Namespace attributeStatNamespace = new Namespace("Stat", attributeNamespace);
            Type attributeType = new Type(typeof(Attribute), attributeNamespace);
            Type attributesType = new Type(typeof(Attributes), attributeNamespace);
            Type charismaAttributeType = new Type(typeof(CharismaAttribute), attributeNamespace);
            Type constitutionAttributeType = new Type(typeof(ConstitutionAttribute), attributeNamespace);
            Type defenseAttributeType = new Type(typeof(DefenseAttribute), attributeNamespace);
            Type dexterityAttributeType = new Type(typeof(DexterityAttribute), attributeNamespace);
            Type intelligenceAttributeType = new Type(typeof(IntelligenceAttribute), attributeNamespace);
            Type luckAttributeType = new Type(typeof(LuckAttribute), attributeNamespace);
            Type perceptionAttributeType = new Type(typeof(PerceptionAttribute), attributeNamespace);
            Type strengthAttributeType = new Type(typeof(StrengthAttribute), attributeNamespace);
            Type willpowerAttributeType = new Type(typeof(WillpowerAttribute), attributeNamespace);
            Type wisdomAttributeType = new Type(typeof(WisdomAttribute), attributeNamespace);
            Type statType = new Type(typeof(Stat), attributeStatNamespace);
            Type statsType = new Type(typeof(Stats), attributeStatNamespace);
            Type agilityStatType = new Type(typeof(AgilityStat), attributeStatNamespace);
            Type alertnessStatType = new Type(typeof(AlertnessStat), attributeStatNamespace);
            Type awarenessStatType = new Type(typeof(AwarenessStat), attributeStatNamespace);
            Type bodyStatType = new Type(typeof(BodyStat), attributeStatNamespace);
            Type brawnStatType = new Type(typeof(BrawnStat), attributeStatNamespace);
            Type cautiousnessStatType = new Type(typeof(CautiousnessStat), attributeStatNamespace);
            Type chanceStatType = new Type(typeof(ChanceStat), attributeStatNamespace);
            Type charmStatType = new Type(typeof(CharmStat), attributeStatNamespace);
            Type egoStatType = new Type(typeof(EgoStat), attributeStatNamespace);
            Type enduranceStatType = new Type(typeof(EnduranceStat), attributeStatNamespace);
            Type fateStatType = new Type(typeof(FateStat), attributeStatNamespace);
            Type fortitudeStatType = new Type(typeof(FortitudeStat), attributeStatNamespace);
            Type fortuneStatType = new Type(typeof(FortuneStat), attributeStatNamespace);
            Type intellectStatType = new Type(typeof(IntellectStat), attributeStatNamespace);
            Type knowledgeStatType = new Type(typeof(KnowledgeStat), attributeStatNamespace);
            Type mightStatType = new Type(typeof(MightStat), attributeStatNamespace);
            Type mindStatType = new Type(typeof(MindStat), attributeStatNamespace);
            Type personalityStatType = new Type(typeof(PersonalityStat), attributeStatNamespace);
            Type powerStatType = new Type(typeof(PowerStat), attributeStatNamespace);
            Type presenceStatType = new Type(typeof(PresenceStat), attributeStatNamespace);
            Type psycheStatType = new Type(typeof(PsycheStat), attributeStatNamespace);
            Type quicknessStatType = new Type(typeof(QuicknessStat), attributeStatNamespace);
            Type recoveryStatType = new Type(typeof(RecoveryStat), attributeStatNamespace);
            Type reflexesStatType = new Type(typeof(ReflexesStat), attributeStatNamespace);
            Type resilienceStatType = new Type(typeof(ResilienceStat), attributeStatNamespace);
            Type resistanceStatType = new Type(typeof(ResistanceStat), attributeStatNamespace);
            Type resolveStatType = new Type(typeof(ResolveStat), attributeStatNamespace);
            Type sanityStatType = new Type(typeof(SanityStat), attributeStatNamespace);
            Type senseStatType = new Type(typeof(SenseStat), attributeStatNamespace);
            Type socialStatType = new Type(typeof(SocialStat), attributeStatNamespace);
            Type spiritStatType = new Type(typeof(SpiritStat), attributeStatNamespace);
            Type staminaStatType = new Type(typeof(StaminaStat), attributeStatNamespace);
            Type vitalityStatType = new Type(typeof(VitalityStat), attributeStatNamespace);
            Type witsStatType = new Type(typeof(WitsStat), attributeStatNamespace);

            Namespace backgroundNamespace = new Namespace("Background", rootNamespace);
            Type backgroundType = new Type(typeof(Background), backgroundNamespace);

            Namespace chanceNamespace = new Namespace("Chance", rootNamespace);
            Type chanceType = new Type(typeof(Chance), chanceNamespace);
            Type seedType = new Type(typeof(Seed<IComparable>), chanceNamespace);

            Namespace coreNamespace = new Namespace("Core", rootNamespace);
            Namespace coreRegistryNamespace = new Namespace("Registry", coreNamespace);
            Type extendedMonoBehaviourType = new Type(typeof(ExtendedMonoBehaviour), coreNamespace);
            Type mainManagerType = new Type(typeof(MainManager), coreNamespace);
            Type instanceType = new Type(typeof(Instance), coreNamespace);
            Type instanceManagerType = new Type(typeof(InstanceManager), coreNamespace);
            Type namespaceType = new Type(typeof(Namespace), coreNamespace);
            Type namespaceManagerType = new Type(typeof(NamespaceManager), coreNamespace);
            Type typeType = new Type(typeof(Type), coreNamespace);
            Type typeManagerType = new Type(typeof(TypeManager), coreNamespace);
            Type namespaceRegistryType = new Type(typeof(Registry<NamespaceIdentifier, Namespace>), coreRegistryNamespace);
            Type typeRegistryType = new Type(typeof(Registry<TypeIdentifier, Type>), coreRegistryNamespace);
            Type instanceRegistryType = new Type(typeof(Registry<InstanceIdentifier, Instance>), coreRegistryNamespace);
            
            Namespace currencyNamespace = new Namespace("Currency", rootNamespace);
            Type coinsType = new Type(typeof(Coins), currencyNamespace);
            Type creditsType = new Type(typeof(Credits), currencyNamespace);
            Type tokensType = new Type(typeof(Tokens), currencyNamespace);
            
            Namespace dataNamespace = new Namespace("Data", rootNamespace);

            Namespace diagnosticNamespace = new Namespace("Diagnostic", rootNamespace);
            Type benchmarkType = new Type(typeof(Benchmark), diagnosticNamespace);

            Namespace enemyNamespace = new Namespace("Enemy", rootNamespace);
            Type enemyType = new Type(typeof(Enemy), enemyNamespace);
            Type smolEnemyType = new Type(typeof(SmolEnemy), enemyNamespace);

            Namespace eventNamespace = new Namespace("Event", rootNamespace);
            Type eventType = new Type(typeof(Event), eventNamespace);
            Type eventListenerType = new Type(typeof(EventListener), eventNamespace);

            Namespace experienceNamespace = new Namespace("Experience", rootNamespace);
            Type iExperienceType = new Type(typeof(IExperience), experienceNamespace);
            Type playerExperienceType = new Type(typeof(PlayerExperience), experienceNamespace);

            Namespace gameNamespace = new Namespace("Game", rootNamespace);
            Type gameType = new Type(typeof(Game), gameNamespace);
            Type gameManagerType = new Type(typeof(GameManager), gameNamespace);
            Type gamesType = new Type(typeof(Games), gameNamespace);

            Namespace generatorNamespace = new Namespace("Generator", rootNamespace);
            Type generatorType = new Type(typeof(Generator), generatorNamespace);
            Type generatorsType = new Type(typeof(Generators), generatorNamespace);
            Type allyStationGeneratorType = new Type(typeof(AllyStationGenerator), generatorNamespace);
            Type asteroidGeneratorType = new Type(typeof(AsteroidGenerator), generatorNamespace);
            Type enemyStationGeneratorType = new Type(typeof(EnemyStationGenerator), generatorNamespace);

            Namespace healthNamespace = new Namespace("Health", rootNamespace);
            Type iHealthType = new Type(typeof(IHealth), healthNamespace);
            Type damageInfoType = new Type(typeof(DamageInfo), healthNamespace);
            Type playerHealthType = new Type(typeof(PlayerHealth), healthNamespace);
            Type allyHealthType = new Type(typeof(AllyHealth), healthNamespace);
            Type allyStationHealthType = new Type(typeof(AllyStationHealth), healthNamespace);
            Type enemyHealthType = new Type(typeof(EnemyHealth), healthNamespace);
            Type enemyStationHealthType = new Type(typeof(EnemyStationHealth), healthNamespace);

            Namespace identifierNamespace = new Namespace("Identifier", rootNamespace);
            Type iIdentifierType = new Type(typeof(IIdentifier), identifierNamespace);
            Type iIdentifiableType = new Type(typeof(IIdentifiable), identifierNamespace);
            Type namespaceIdentifierType = new Type(typeof(NamespaceIdentifier), identifierNamespace);
            Type typeIdentifierType = new Type(typeof(TypeIdentifier), identifierNamespace);
            Type instanceIdentifierType = new Type(typeof(InstanceIdentifier), identifierNamespace);

            Namespace indicatorNamespace = new Namespace("Indicator", rootNamespace);
            Type damageIndicatorType = new Type(typeof(DamageIndicator), indicatorNamespace);

            Namespace inventoryNamespace = new Namespace("Inventory", rootNamespace);
            Type playerInventoryType = new Type(typeof(PlayerInventory), inventoryNamespace);
            Type allyStationInventoryType = new Type(typeof(AllyStationInventory), inventoryNamespace);
            Type enemyStationInventoryType = new Type(typeof(EnemyStationInventory), inventoryNamespace);

            Namespace itemNamespace = new Namespace("Item", rootNamespace);
            Type itemType = new Type(typeof(Item), itemNamespace);
            Type itemObjectType = new Type(typeof(ItemObject), itemNamespace);
            Type itemContainerType = new Type(typeof(ItemContainer), itemNamespace);
            Type itemContainerSlotType = new Type(typeof(ItemContainerSlot), itemNamespace);
            Type amountableItemType = new Type(typeof(AmountableItem), itemNamespace);
            Type amountableItemObject = new Type(typeof(AmountableItemObject), itemNamespace);
            Type countableItemType = new Type(typeof(CountableItem), itemNamespace);
            Type countableItemObjectType = new Type(typeof(CountableItemObject), itemNamespace);
            Type uniqueItemType = new Type(typeof(UniqueItem), itemNamespace);
            Type uniqueItemObjectType = new Type(typeof(UniqueItemObject), itemNamespace);
            Type upgradableItemType = new Type(typeof(UpgradableItem), itemNamespace);
            Type upgradableItemObjectType = new Type(typeof(UpgradableItemObject), itemNamespace);
            Type upgradeSetType = new Type(typeof(UpgradeSet), itemNamespace);
            Type iItemUpgraderType = new Type(typeof(IItemUpgrader), itemNamespace);
            Type resourceItemType = new Type(typeof(ResourceItem), itemNamespace);
            Type resourceItemObjectType = new Type(typeof(ResourceItemObject), itemNamespace);
            Type weaponItemType = new Type(typeof(WeaponItem), itemNamespace);
            Type weaponItemObjectType = new Type(typeof(WeaponItemObject), itemNamespace);
            Type weaponItemContainerType = new Type(typeof(WeaponItemContainer), itemNamespace);
            Type chargedPlasmaLauncherWeaponItemType = new Type(typeof(ChargedPlasmaLauncherWeaponItem), itemNamespace);
            Type chargedPlasmaLauncherWeaponItemObjectType = new Type(typeof(ChargedPlasmaLauncherWeaponItemObject), itemNamespace);
            Type freezeRayWeaponItemType = new Type(typeof(FreezeRayWeaponItem), itemNamespace);
            Type freezeRayWeaponItemObjectType = new Type(typeof(FreezeRayWeaponItemObject), itemNamespace);
            Type laserEmitterWeaponItemType = new Type(typeof(LaserEmitterWeaponItem), itemNamespace);
            Type laserEmitterWeaponItemObjectType = new Type(typeof(LaserEmitterWeaponItemObject), itemNamespace);
            Type multiplexerWeaponItemType = new Type(typeof(MultiplexerWeaponItem), itemNamespace);
            Type multiplexerWeaponItemObjectType = new Type(typeof(MultiplexerWeaponItemObject), itemNamespace);

            Namespace mainMenuNamespace = new Namespace("MainMenu", rootNamespace);
            Type mainMenuManagerType = new Type(typeof(MainMenuManager), mainMenuNamespace);

            Namespace mathNamespace = new Namespace("Math", rootNamespace);
            Namespace mathMapNamespace = new Namespace("Map", mathNamespace);
            Type floatMap2DType = new Type(typeof(FloatMap2D), mathMapNamespace);

            Namespace missionNamespace = new Namespace("Mission", rootNamespace);
            Namespace missionRewardNamespace = new Namespace("Reward", missionNamespace);
            Namespace missionTargetNamespace = new Namespace("Target", missionNamespace);
            Namespace missionTaskNamespace = new Namespace("Task", missionNamespace);
            Namespace missionTriggerNamespace = new Namespace("Trigger", missionNamespace);
            Type conquerStationMissionType = new Type(typeof(ConquerStationMission), missionNamespace);
            Type missionType = new Type(typeof(Mission), missionNamespace);
            Type missionManagerType = new Type(typeof(MissionManager), missionNamespace);
            Type missionProviderType = new Type(typeof(MissionProvider), missionNamespace);
            Type missionRarityType = new Type(typeof(MissionRarity), missionNamespace);
            Type missionStateType = new Type(typeof(MissionState), missionNamespace);
            Type creditsMissionRewardType = new Type(typeof(LooCast.Mission.Reward.CreditsMissionReward), missionRewardNamespace);
            Type itemMissionRewardType = new Type(typeof(LooCast.Mission.Reward.ItemMissionReward), missionRewardNamespace);
            Type missionRewardType = new Type(typeof(LooCast.Mission.Reward.MissionReward), missionRewardNamespace);
            Type reputationMissionRewardType = new Type(typeof(LooCast.Mission.Reward.ReputationMissionReward), missionRewardNamespace);
            Type missionTargetType = new Type(typeof(MissionTarget), missionTargetNamespace);
            Type iMissionTaskLockStateType = new Type(typeof(IMissionTaskLockState), missionTaskNamespace);
            Type lockedMissionTaskLockStateType = new Type(typeof(LockedMissionTaskLockState), missionTaskNamespace);
            Type missionTaskType = new Type(typeof(LooCast.Mission.Task.MissionTask), missionTaskNamespace);
            Type unlockedMissionTaskLockStateType = new Type(typeof(UnlockedMissionTaskLockState), missionTaskNamespace);
            Type missionTriggerType = new Type(typeof(MissionTrigger), missionTriggerNamespace);

            Namespace movementNamespace = new Namespace("Movement", rootNamespace);
            Namespace movementEffectNamespace = new Namespace("Effect", movementNamespace);
            Type iMovementType = new Type(typeof(IMovement), movementNamespace);
            Type allyMovementType = new Type(typeof(AllyMovement), movementNamespace);
            Type enemyMovementType = new Type(typeof(EnemyMovement), movementNamespace);
            Type playerMovementType = new Type(typeof(PlayerMovement), movementNamespace);
            Type freezeMovementEffectType = new Type(typeof(FreezeMovementEffect), movementEffectNamespace);
            Type movementEffectType = new Type(typeof(MovementEffect), movementEffectNamespace);

            Namespace noiseNamespace = new Namespace("Noise", rootNamespace);
            Type fastNoiseLiteType = new Type(typeof(FastNoiseLite), noiseNamespace);
            Type perlinNoiseType = new Type(typeof(PerlinNoise), noiseNamespace);
            Type voronoiNoiseType = new Type(typeof(VoronoiNoise), noiseNamespace);

            Namespace observerNamespace = new Namespace("Observer", rootNamespace);
            Type universeObserverType = new Type(typeof(UniverseObserver), observerNamespace);

            Namespace orbNamespace = new Namespace("Orb", rootNamespace);
            Type experienceOrbType = new Type(typeof(ExperienceOrb), orbNamespace);
            Type magnetOrbType = new Type(typeof(MagnetOrb), orbNamespace);

            Namespace particleNamespace = new Namespace("Particle", rootNamespace);
            Type particleSystemType = new Type(typeof(ParticleSystem), particleNamespace);

            Namespace playerNamespace = new Namespace("Player", rootNamespace);
            Type playerType = new Type(typeof(Player), playerNamespace);

            Namespace projectileNamespace = new Namespace("Projectile", rootNamespace);
            Type projectileType = new Type(typeof(Projectile), projectileNamespace);
            Type chargedPlasmaProjectileType = new Type(typeof(ChargedPlasmaProjectile), projectileNamespace);
            Type laserProjectileType = new Type(typeof(LaserProjectile), projectileNamespace);
            Type multiplexerProjectileType = new Type(typeof(MultiplexerProjectile), projectileNamespace);
            Type multiplexerFragmentProjectileType = new Type(typeof(MultiplexerFragmentProjectile), projectileNamespace);

            Namespace randomNamespace = new Namespace("Random", rootNamespace);
            Type randomType = new Type(typeof(Random), randomNamespace);
            Type seededRandomType = new Type(typeof(SeededRandom), randomNamespace);

            Namespace resourceNamespace = new Namespace("Resource", rootNamespace);
            Type resourceType = new Type(typeof(Resource), resourceNamespace);

            Namespace soundNamespace = new Namespace("Sound", rootNamespace);
            Type soundType = new Type(typeof(Sound), soundNamespace);
            Type soundHandlerType = new Type(typeof(SoundHandler), soundNamespace);
            Type gameSoundHandlerType = new Type(typeof(GameSoundHandler), soundNamespace);
            Type menuSoundHandlerType = new Type(typeof(MenuSoundHandler), soundNamespace);

            Namespace spawnerNamespace = new Namespace("Spawner", rootNamespace);
            Type spawnerType = new Type(typeof(Spawner), spawnerNamespace);
            Type allySpawnerType = new Type(typeof(AllySpawner), spawnerNamespace);
            Type enemySpawnerType = new Type(typeof(EnemySpawner), spawnerNamespace);

            Namespace stateMachineNamespace = new Namespace("StateMachine", rootNamespace);
            Type finiteStateMachineType = new Type(typeof(FiniteStateMachine<object>), stateMachineNamespace);
            Type stateType = new Type(typeof(State<object>), stateMachineNamespace);

            Namespace stationNamespace = new Namespace("Station", rootNamespace);
            Type stationType = new Type(typeof(Station), stationNamespace);
            Type allyStationType = new Type(typeof(AllyStation), stationNamespace);
            Type enemyStationType = new Type(typeof(EnemyStation), stationNamespace);

            Namespace statisticNamespace = new Namespace("Statistic", rootNamespace);
            Type killsStatisticType = new Type(typeof(KillsStatistic), statisticNamespace);

            Namespace steamworksNamespace = new Namespace("Steamworks", rootNamespace);
            Type steamManagerType = new Type(typeof(SteamManager), steamworksNamespace);

            Namespace targetNamespace = new Namespace("Target", rootNamespace);
            Type targetType = new Type(typeof(Target), targetNamespace);

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
            Type versionInfoType = new Type(typeof(VersionInfo), uiNamespace);

            Namespace universeNamespace = new Namespace("Universe", rootNamespace);
            Type universeType = new Type(typeof(Universe), universeNamespace);

            Namespace utilNamespace = new Namespace("Util", rootNamespace);
            Namespace utilCollectionsNamespace = new Namespace("Collections", utilNamespace);
            Namespace utilCollectionsConcurrentNamespace = new Namespace("Concurrent", utilCollectionsNamespace);
            Namespace utilCollectionsGenericNamespace = new Namespace("Generic", utilCollectionsNamespace);
            Type colorUtilType = new Type(typeof(ColorUtil), utilNamespace);
            Type constantsType = new Type(typeof(LooCast.Util.Constants), utilNamespace);
            Type extensionMethodsType = new Type(typeof(ExtensionMethods), utilNamespace);
            Type lerpFollowerType = new Type(typeof(LerpFollower), utilNamespace);
            Type rectTransformUtilType = new Type(typeof(RectTransformUtil), utilNamespace);
            Type screenShakeType = new Type(typeof(ScreenShake), utilNamespace);
            Type serializationUtilType = new Type(typeof(SerializationUtil), utilNamespace);
            Type targetingUtilType = new Type(typeof(TargetingUtil), utilNamespace);
            Type teamUtilType = new Type(typeof(TeamUtil), utilNamespace);
            Type textureUtilType = new Type(typeof(TextureUtil), utilNamespace);
            Type timerUtilType = new Type(typeof(TimerUtil), utilNamespace);
            Type concurrentSerializableDictionaryType = new Type(typeof(ConcurrentSerializableDictionary<object, object>), utilCollectionsConcurrentNamespace);
            Type serializableDictionaryType = new Type(typeof(SerializableDictionary<object, object>), utilCollectionsGenericNamespace);
            Type serializableListType = new Type(typeof(SerializableList<object>), utilCollectionsGenericNamespace);

            Namespace variableNamespace = new Namespace("Variable", rootNamespace);
            Type variableType = new Type(typeof(Variable<object>), variableNamespace);
            Type computedVariableType = new Type(typeof(ComputedVariable<object>), variableNamespace);
            Type computedVariableUtilType = new Type(typeof(ComputedVariableUtil), variableNamespace);
            Type increaseType = new Type(typeof(Increase), variableNamespace);
            Type multiplierType = new Type(typeof(Multiplier), variableNamespace);
            Type temporaryIncreaseType = new Type(typeof(TemporaryIncrease), variableNamespace);
            Type temporaryMultiplierType = new Type(typeof(TemporaryMultiplier), variableNamespace);
            Type boolVariableType = new Type(typeof(BoolVariable), variableNamespace);
            Type floatVariableType = new Type(typeof(FloatVariable), variableNamespace);
            Type floatComputedVariableType = new Type(typeof(FloatComputedVariable), variableNamespace);
            Type intVariableType = new Type(typeof(IntVariable), variableNamespace);
            Type intComputedVariableType = new Type(typeof(IntComputedVariable), variableNamespace);
            Type stringVariableType = new Type(typeof(StringVariable), variableNamespace);

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
