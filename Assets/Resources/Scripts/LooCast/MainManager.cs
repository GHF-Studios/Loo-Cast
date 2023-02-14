using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace LooCast
{
    using LooCast.Game;

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
        public static Games Games => games;
        public static Game.Game GameToBeLoaded => gameToBeLoaded;    // TODO: Implement this
        public static CoreModuleManager[] CoreModuleManagers { get; private set; }
        #endregion

        #region Static Fields
        private static MainManager instance;
        private static Games games;
        private static Game.Game gameToBeLoaded;
        public static float saveInterval = 30.0f;
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

        public static void CreateNewGame(string gameName, Universe.Universe.GenerationSettings generationSettings)
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
                Game.Game game = games.GetGame(gameName);
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

            Game.Game game = games.GetGame(gameName);
            Game.Game.DeleteGame(game);
        }

        public static void RenameGame(string oldGameName, string newGameName)
        {
            if (GameManager.Initialized && GameManager.Instance.CurrentGame.Name == oldGameName)
            {
                throw new Exception("Cannot rename Game when it is loaded!");
            }

            Game.Game game = games.GetGame(oldGameName);
            Game.Game.Rename(game, newGameName);
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
        #endregion

        #region Methods

        #region Initialization Phases
        private void EarlyPreInitialize()
        {
            string activeSceneName = SceneManager.GetActiveScene().name;
            IsEarlyPreInitializing = true;
            Debug.Log($"[MainManager] Starting Early Pre-Initialization in Scene '{activeSceneName}'.");

            #region Early Pre-Initialization

            #region Main Manager
            Namespace rootNamespace = new Namespace("LooCast");
            NamespaceManager.Instance.RegisterNamespace(rootNamespace);
            NamespaceManager.Instance.GetNamespace(new NamespaceIdentifier("LooCast"));

            // TODO: Fetch CoreModuleManagers, ordered by their Dependencies(index 0 is Base Mod Core Module Manager, 1 is Mod Core Module Manager, 2 is Mod Extension Core Module Manager, 3 is Mod Extension Extension Core Module Manager, etc.)
            #endregion

            #region Core Module Managers
            foreach (CoreModuleManager coreModuleManager in CoreModuleManagers)
            {
                coreModuleManager.EarlyPreInitialize();
            }
            #endregion

            #region DEPRECATED! [TO BE MOVED!]
            Namespace aiNamespace = new Namespace("AI", rootNamespace);
            Type allyAIType = new Type(typeof(AllyAI), aiNamespace);
            Type enemyAIType = new Type(typeof(EnemyAI), aiNamespace);

            Namespace allyNamespace = new Namespace("Ally", rootNamespace);
            Type allyType = new Type(typeof(Ally.Ally), allyNamespace);
            Type smolAllyType = new Type(typeof(SmolAlly), allyNamespace);

            Namespace aoeNamespace = new Namespace("AOE", rootNamespace);
            Type freezeZoneType = new Type(typeof(FreezeZone), aoeNamespace);

            Namespace arcNamespace = new Namespace("Arc", rootNamespace);
            Type arcType = new Type(typeof(Arc.Arc), arcNamespace);
            Type arcSegmentType = new Type(typeof(ArcSegment), arcNamespace);

            Namespace asteroidNamespace = new Namespace("Asteroid", rootNamespace);
            Type asteroidType = new Type(typeof(Asteroid.Asteroid), asteroidNamespace);

            Namespace attributeNamespace = new Namespace("Attribute", rootNamespace);
            Namespace attributeStatNamespace = new Namespace("Stat", attributeNamespace);
            Type attributeType = new Type(typeof(Attribute.Attribute), attributeNamespace);
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
            Type backgroundType = new Type(typeof(Background.Background), backgroundNamespace);

            Namespace chanceNamespace = new Namespace("Chance", rootNamespace);
            Type chanceType = new Type(typeof(Chance.Chance), chanceNamespace);
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
            Type enemyType = new Type(typeof(Enemy.Enemy), enemyNamespace);
            Type smolEnemyType = new Type(typeof(SmolEnemy), enemyNamespace);

            Namespace eventNamespace = new Namespace("Event", rootNamespace);
            Type eventType = new Type(typeof(Event.Event), eventNamespace);
            Type eventListenerType = new Type(typeof(EventListener), eventNamespace);

            Namespace experienceNamespace = new Namespace("Experience", rootNamespace);
            Type iExperienceType = new Type(typeof(IExperience), experienceNamespace);
            Type playerExperienceType = new Type(typeof(PlayerExperience), experienceNamespace);

            Namespace gameNamespace = new Namespace("Game", rootNamespace);
            Type gameType = new Type(typeof(Game.Game), gameNamespace);
            Type gameManagerType = new Type(typeof(GameManager), gameNamespace);
            Type gamesType = new Type(typeof(Games), gameNamespace);

            Namespace generatorNamespace = new Namespace("Generator", rootNamespace);
            Type generatorType = new Type(typeof(Generator.Generator), generatorNamespace);
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
            Type itemType = new Type(typeof(Item.Item), itemNamespace);
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
            Type playerType = new Type(typeof(Player.Player), playerNamespace);

            Namespace projectileNamespace = new Namespace("Projectile", rootNamespace);
            Type projectileType = new Type(typeof(Projectile.Projectile), projectileNamespace);
            Type chargedPlasmaProjectileType = new Type(typeof(ChargedPlasmaProjectile), projectileNamespace);
            Type laserProjectileType = new Type(typeof(LaserProjectile), projectileNamespace);
            Type multiplexerProjectileType = new Type(typeof(MultiplexerProjectile), projectileNamespace);
            Type multiplexerFragmentProjectileType = new Type(typeof(MultiplexerFragmentProjectile), projectileNamespace);

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

            namespaceManager.RegisterNamespace(aiNamespace);
            typeManager.RegisterType(allyAIType);
            typeManager.RegisterType(enemyAIType);

            namespaceManager.RegisterNamespace(allyNamespace);
            typeManager.RegisterType(allyType);
            typeManager.RegisterType(smolAllyType);

            namespaceManager.RegisterNamespace(aoeNamespace);
            typeManager.RegisterType(freezeZoneType);

            namespaceManager.RegisterNamespace(arcNamespace);
            typeManager.RegisterType(arcType);
            typeManager.RegisterType(arcSegmentType);

            namespaceManager.RegisterNamespace(asteroidNamespace);
            typeManager.RegisterType(asteroidType);

            namespaceManager.RegisterNamespace(attributeNamespace);
            namespaceManager.RegisterNamespace(attributeStatNamespace);
            typeManager.RegisterType(attributeType);
            typeManager.RegisterType(attributesType);
            typeManager.RegisterType(charismaAttributeType);
            typeManager.RegisterType(constitutionAttributeType);
            typeManager.RegisterType(defenseAttributeType);
            typeManager.RegisterType(dexterityAttributeType);
            typeManager.RegisterType(intelligenceAttributeType);
            typeManager.RegisterType(luckAttributeType);
            typeManager.RegisterType(perceptionAttributeType);
            typeManager.RegisterType(strengthAttributeType);
            typeManager.RegisterType(willpowerAttributeType);
            typeManager.RegisterType(wisdomAttributeType);
            typeManager.RegisterType(statType);
            typeManager.RegisterType(statsType);
            typeManager.RegisterType(agilityStatType);
            typeManager.RegisterType(alertnessStatType);
            typeManager.RegisterType(awarenessStatType);
            typeManager.RegisterType(bodyStatType);
            typeManager.RegisterType(brawnStatType);
            typeManager.RegisterType(cautiousnessStatType);
            typeManager.RegisterType(chanceStatType);
            typeManager.RegisterType(charmStatType);
            typeManager.RegisterType(egoStatType);
            typeManager.RegisterType(enduranceStatType);
            typeManager.RegisterType(fateStatType);
            typeManager.RegisterType(fortitudeStatType);
            typeManager.RegisterType(fortuneStatType);
            typeManager.RegisterType(intellectStatType);
            typeManager.RegisterType(knowledgeStatType);
            typeManager.RegisterType(mightStatType);
            typeManager.RegisterType(mindStatType);
            typeManager.RegisterType(personalityStatType);
            typeManager.RegisterType(powerStatType);
            typeManager.RegisterType(presenceStatType);
            typeManager.RegisterType(psycheStatType);
            typeManager.RegisterType(quicknessStatType);
            typeManager.RegisterType(recoveryStatType);
            typeManager.RegisterType(reflexesStatType);
            typeManager.RegisterType(resilienceStatType);
            typeManager.RegisterType(resistanceStatType);
            typeManager.RegisterType(resolveStatType);
            typeManager.RegisterType(sanityStatType);
            typeManager.RegisterType(senseStatType);
            typeManager.RegisterType(socialStatType);
            typeManager.RegisterType(spiritStatType);
            typeManager.RegisterType(staminaStatType);
            typeManager.RegisterType(vitalityStatType);
            typeManager.RegisterType(witsStatType);

            namespaceManager.RegisterNamespace(backgroundNamespace);
            typeManager.RegisterType(backgroundType);

            namespaceManager.RegisterNamespace(chanceNamespace);
            typeManager.RegisterType(chanceType);
            typeManager.RegisterType(seedType);

            namespaceManager.RegisterNamespace(coreNamespace);
            namespaceManager.RegisterNamespace(coreRegistryNamespace);
            typeManager.RegisterType(extendedMonoBehaviourType);
            typeManager.RegisterType(mainManagerType);
            typeManager.RegisterType(instanceType);
            typeManager.RegisterType(instanceManagerType);
            typeManager.RegisterType(namespaceType);
            typeManager.RegisterType(namespaceManagerType);
            typeManager.RegisterType(typeType);
            typeManager.RegisterType(typeManagerType);
            typeManager.RegisterType(namespaceRegistryType);
            typeManager.RegisterType(typeRegistryType);
            typeManager.RegisterType(instanceRegistryType);

            namespaceManager.RegisterNamespace(currencyNamespace);
            typeManager.RegisterType(coinsType);
            typeManager.RegisterType(creditsType);
            typeManager.RegisterType(tokensType);

            namespaceManager.RegisterNamespace(dataNamespace);

            namespaceManager.RegisterNamespace(diagnosticNamespace);
            typeManager.RegisterType(benchmarkType);

            namespaceManager.RegisterNamespace(enemyNamespace);
            typeManager.RegisterType(enemyType);
            typeManager.RegisterType(smolEnemyType);

            namespaceManager.RegisterNamespace(eventNamespace);
            typeManager.RegisterType(eventType);
            typeManager.RegisterType(eventListenerType);

            namespaceManager.RegisterNamespace(experienceNamespace);
            typeManager.RegisterType(iExperienceType);
            typeManager.RegisterType(playerExperienceType);

            namespaceManager.RegisterNamespace(gameNamespace);
            typeManager.RegisterType(gameType);
            typeManager.RegisterType(gameManagerType);
            typeManager.RegisterType(gamesType);

            namespaceManager.RegisterNamespace(generatorNamespace);
            typeManager.RegisterType(generatorType);
            typeManager.RegisterType(generatorsType);
            typeManager.RegisterType(allyStationGeneratorType);
            typeManager.RegisterType(asteroidGeneratorType);
            typeManager.RegisterType(enemyStationGeneratorType);

            namespaceManager.RegisterNamespace(healthNamespace);
            typeManager.RegisterType(iHealthType);
            typeManager.RegisterType(damageInfoType);
            typeManager.RegisterType(playerHealthType);
            typeManager.RegisterType(allyHealthType);
            typeManager.RegisterType(allyStationHealthType);
            typeManager.RegisterType(enemyHealthType);
            typeManager.RegisterType(enemyStationHealthType);

            namespaceManager.RegisterNamespace(identifierNamespace);
            typeManager.RegisterType(iIdentifierType);
            typeManager.RegisterType(iIdentifiableType);
            typeManager.RegisterType(namespaceIdentifierType);
            typeManager.RegisterType(typeIdentifierType);
            typeManager.RegisterType(instanceIdentifierType);

            namespaceManager.RegisterNamespace(indicatorNamespace);
            typeManager.RegisterType(damageIndicatorType);

            namespaceManager.RegisterNamespace(inventoryNamespace);
            typeManager.RegisterType(playerInventoryType);
            typeManager.RegisterType(allyStationInventoryType);
            typeManager.RegisterType(enemyStationInventoryType);

            namespaceManager.RegisterNamespace(itemNamespace);
            typeManager.RegisterType(itemType);
            typeManager.RegisterType(itemObjectType);
            typeManager.RegisterType(itemContainerType);
            typeManager.RegisterType(itemContainerSlotType);
            typeManager.RegisterType(amountableItemType);
            typeManager.RegisterType(amountableItemObject);
            typeManager.RegisterType(countableItemType);
            typeManager.RegisterType(countableItemObjectType);
            typeManager.RegisterType(uniqueItemType);
            typeManager.RegisterType(uniqueItemObjectType);
            typeManager.RegisterType(upgradableItemType);
            typeManager.RegisterType(upgradableItemObjectType);
            typeManager.RegisterType(upgradeSetType);
            typeManager.RegisterType(iItemUpgraderType);
            typeManager.RegisterType(resourceItemType);
            typeManager.RegisterType(resourceItemObjectType);
            typeManager.RegisterType(weaponItemType);
            typeManager.RegisterType(weaponItemObjectType);
            typeManager.RegisterType(weaponItemContainerType);
            typeManager.RegisterType(chargedPlasmaLauncherWeaponItemType);
            typeManager.RegisterType(chargedPlasmaLauncherWeaponItemObjectType);
            typeManager.RegisterType(freezeRayWeaponItemType);
            typeManager.RegisterType(freezeRayWeaponItemObjectType);
            typeManager.RegisterType(laserEmitterWeaponItemType);
            typeManager.RegisterType(laserEmitterWeaponItemObjectType);
            typeManager.RegisterType(multiplexerWeaponItemType);
            typeManager.RegisterType(multiplexerWeaponItemObjectType);

            namespaceManager.RegisterNamespace(mainMenuNamespace);
            typeManager.RegisterType(mainMenuManagerType);

            namespaceManager.RegisterNamespace(mathNamespace);
            namespaceManager.RegisterNamespace(mathMapNamespace);
            typeManager.RegisterType(floatMap2DType);

            namespaceManager.RegisterNamespace(missionNamespace);
            namespaceManager.RegisterNamespace(missionRewardNamespace);
            namespaceManager.RegisterNamespace(missionTargetNamespace);
            namespaceManager.RegisterNamespace(missionTaskNamespace);
            namespaceManager.RegisterNamespace(missionTriggerNamespace);
            typeManager.RegisterType(conquerStationMissionType);
            typeManager.RegisterType(missionType);
            typeManager.RegisterType(missionManagerType);
            typeManager.RegisterType(missionProviderType);
            typeManager.RegisterType(missionRarityType);
            typeManager.RegisterType(missionStateType);
            typeManager.RegisterType(creditsMissionRewardType);
            typeManager.RegisterType(itemMissionRewardType);
            typeManager.RegisterType(missionRewardType);
            typeManager.RegisterType(reputationMissionRewardType);
            typeManager.RegisterType(missionTargetType);
            typeManager.RegisterType(iMissionTaskLockStateType);
            typeManager.RegisterType(lockedMissionTaskLockStateType);
            typeManager.RegisterType(missionTaskType);
            typeManager.RegisterType(unlockedMissionTaskLockStateType);
            typeManager.RegisterType(missionTriggerType);

            namespaceManager.RegisterNamespace(movementNamespace);
            namespaceManager.RegisterNamespace(movementEffectNamespace);
            typeManager.RegisterType(iMovementType);
            typeManager.RegisterType(allyMovementType);
            typeManager.RegisterType(enemyMovementType);
            typeManager.RegisterType(playerMovementType);
            typeManager.RegisterType(freezeMovementEffectType);
            typeManager.RegisterType(movementEffectType);

            namespaceManager.RegisterNamespace(noiseNamespace);
            typeManager.RegisterType(fastNoiseLiteType);
            typeManager.RegisterType(perlinNoiseType);
            typeManager.RegisterType(voronoiNoiseType);

            namespaceManager.RegisterNamespace(observerNamespace);
            typeManager.RegisterType(universeObserverType);

            namespaceManager.RegisterNamespace(orbNamespace);
            typeManager.RegisterType(experienceOrbType);
            typeManager.RegisterType(magnetOrbType);

            namespaceManager.RegisterNamespace(particleNamespace);
            typeManager.RegisterType(particleSystemType);

            namespaceManager.RegisterNamespace(playerNamespace);
            typeManager.RegisterType(playerType);

            namespaceManager.RegisterNamespace(projectileNamespace);
            typeManager.RegisterType(projectileType);
            typeManager.RegisterType(chargedPlasmaProjectileType);
            typeManager.RegisterType(laserProjectileType);
            typeManager.RegisterType(multiplexerProjectileType);
            typeManager.RegisterType(multiplexerFragmentProjectileType);

            namespaceManager.RegisterNamespace(randomNamespace);
            typeManager.RegisterType(randomType);
            typeManager.RegisterType(seededRandomType);

            namespaceManager.RegisterNamespace(resourceNamespace);
            typeManager.RegisterType(resourceType);

            namespaceManager.RegisterNamespace(soundNamespace);
            typeManager.RegisterType(soundType);
            typeManager.RegisterType(soundHandlerType);
            typeManager.RegisterType(gameSoundHandlerType);
            typeManager.RegisterType(menuSoundHandlerType);

            namespaceManager.RegisterNamespace(spawnerNamespace);
            typeManager.RegisterType(spawnerType);
            typeManager.RegisterType(allySpawnerType);
            typeManager.RegisterType(enemySpawnerType);

            namespaceManager.RegisterNamespace(stateMachineNamespace);
            typeManager.RegisterType(finiteStateMachineType);
            typeManager.RegisterType(stateType);

            namespaceManager.RegisterNamespace(stationNamespace);
            typeManager.RegisterType(stationType);
            typeManager.RegisterType(allyStationType);
            typeManager.RegisterType(enemyStationType);

            namespaceManager.RegisterNamespace(statisticNamespace);
            typeManager.RegisterType(killsStatisticType);

            namespaceManager.RegisterNamespace(steamworksNamespace);
            typeManager.RegisterType(steamManagerType);

            namespaceManager.RegisterNamespace(targetNamespace);
            typeManager.RegisterType(targetType);

            namespaceManager.RegisterNamespace(testNamespace);
            typeManager.RegisterType(mapDisplayType);
            typeManager.RegisterType(perlinMapGeneratorType);
            typeManager.RegisterType(perlinMapGeneratorGPUType);
            typeManager.RegisterType(voronoiMapGeneratorType);
            typeManager.RegisterType(voronoiMapGeneratorGPUType);

            namespaceManager.RegisterNamespace(uiNamespace);
            namespaceManager.RegisterNamespace(uiBarNamespace);
            namespaceManager.RegisterNamespace(uiButtonNamespace);
            namespaceManager.RegisterNamespace(uiCanvasNamespace);
            namespaceManager.RegisterNamespace(uiCursorNamespace);
            namespaceManager.RegisterNamespace(uiHudNamespace);
            namespaceManager.RegisterNamespace(uiInspectorNamespace);
            namespaceManager.RegisterNamespace(uiInventoryNamespace);
            namespaceManager.RegisterNamespace(uiLevelNamespace);
            namespaceManager.RegisterNamespace(uiPanelNamespace);
            namespaceManager.RegisterNamespace(uiRewardNamespace);
            namespaceManager.RegisterNamespace(uiScreenNamespace);
            namespaceManager.RegisterNamespace(uiSliderNamespace);
            namespaceManager.RegisterNamespace(uiTabNamespace);
            namespaceManager.RegisterNamespace(uiTaskNamespace);
            namespaceManager.RegisterNamespace(uiTimerNamespace);
            namespaceManager.RegisterNamespace(uiTitleNamespace);
            namespaceManager.RegisterNamespace(uiValueNamespace);
            typeManager.RegisterType(uiVersionInfoType);
            typeManager.RegisterType(uiBarType);
            typeManager.RegisterType(uiEnergyBarType);
            typeManager.RegisterType(uiExperienceBarType);
            typeManager.RegisterType(uiHealthBarType);
            typeManager.RegisterType(uiButtonType);
            typeManager.RegisterType(uiAttributeSetButtonType);
            typeManager.RegisterType(uiCreateGameButtonType);
            typeManager.RegisterType(uiHardresetButtonType);
            typeManager.RegisterType(uiLoadGameButtonType);
            typeManager.RegisterType(uiLoadMainMenuButtonType);
            typeManager.RegisterType(uiMissionButtonType);
            typeManager.RegisterType(uiStatSetButtonType);
            typeManager.RegisterType(uiTabButtonType);
            typeManager.RegisterType(uiCanvasType);
            typeManager.RegisterType(uiGameCanvasType);
            typeManager.RegisterType(uiInterfaceCanvasType);
            typeManager.RegisterType(uiMainMenuCanvasType);
            typeManager.RegisterType(uiScreenSpaceCameraCanvasType);
            typeManager.RegisterType(uiScreenSpaceOverlayCanvasType);
            typeManager.RegisterType(uiWorldSpaceCanvasType);
            typeManager.RegisterType(uiAsteroidCursorType);
            typeManager.RegisterType(uiMissionButtonCursorType);
            typeManager.RegisterType(uiHUDType);
            typeManager.RegisterType(uiActiveMissionInspectorType);
            typeManager.RegisterType(uiAsteroidInspectorType);
            typeManager.RegisterType(uiInventoryHotbarType);
            typeManager.RegisterType(uiInventoryItemType);
            typeManager.RegisterType(uiInventorySlotType);
            typeManager.RegisterType(uiInventorySlotCursorType);
            typeManager.RegisterType(uiLevelType);
            typeManager.RegisterType(uiAttributeLevelType);
            typeManager.RegisterType(uiStatLeveltype);
            typeManager.RegisterType(uiPanelType);
            typeManager.RegisterType(uiAsteroidInfoPanelType);
            typeManager.RegisterType(uiAsteroidResourceDeposistsPanelType);
            typeManager.RegisterType(uiStationBlackmarketPanelType);
            typeManager.RegisterType(uiStationHUBPanelType);
            typeManager.RegisterType(uiStationManufacturingPanelType);
            typeManager.RegisterType(uiStationMarketPanelType);
            typeManager.RegisterType(uiStationMissionPanelType);
            typeManager.RegisterType(uiStationUpgradesPanelType);
            typeManager.RegisterType(uiMissionRewardType);
            typeManager.RegisterType(uiCreditsMissionRewardType);
            typeManager.RegisterType(uiItemMissionRewardType);
            typeManager.RegisterType(uiReputationMissionRewardType);
            typeManager.RegisterType(uiScreenType);
            typeManager.RegisterType(uiDeathScreenType);
            typeManager.RegisterType(uiLoadGameScreenType);
            typeManager.RegisterType(uiLoadingScreenType);
            typeManager.RegisterType(uiMainScreenType);
            typeManager.RegisterType(uiNewGameScreenType);
            typeManager.RegisterType(uiPauseScreenType);
            typeManager.RegisterType(uiSettingsScreenType);
            typeManager.RegisterType(uiStationScreenType);
            typeManager.RegisterType(uiStatsScreenType);
            typeManager.RegisterType(uiSliderType);
            typeManager.RegisterType(uiDifficultySliderType);
            typeManager.RegisterType(uiVolumeSliderType);
            typeManager.RegisterType(uiEffectVolumeSliderType);
            typeManager.RegisterType(uiMasterVolumeSliderType);
            typeManager.RegisterType(uiMusicVolumeSliderType);
            typeManager.RegisterType(uiUIVolumeSliderType);
            typeManager.RegisterType(uiTabType);
            typeManager.RegisterType(uiTabGroupType);
            typeManager.RegisterType(uiMissionTaskType);
            typeManager.RegisterType(uiMissionTaskContainerType);
            typeManager.RegisterType(uiRoundTimerType);
            typeManager.RegisterType(uiTitleType);
            typeManager.RegisterType(uiValueType);
            typeManager.RegisterType(uiCoinsValueType);
            typeManager.RegisterType(uiExperienceLevelValueType);
            typeManager.RegisterType(uiStatValueType);
            typeManager.RegisterType(uiTokensValueType);

            namespaceManager.RegisterNamespace(universeNamespace);
            typeManager.RegisterType(universeType);

            namespaceManager.RegisterNamespace(utilNamespace);
            namespaceManager.RegisterNamespace(utilCollectionsNamespace);
            namespaceManager.RegisterNamespace(utilCollectionsConcurrentNamespace);
            namespaceManager.RegisterNamespace(utilCollectionsGenericNamespace);
            typeManager.RegisterType(colorUtilType);
            typeManager.RegisterType(constantsType);
            typeManager.RegisterType(extensionMethodsType);
            typeManager.RegisterType(lerpFollowerType);
            typeManager.RegisterType(rectTransformUtilType);
            typeManager.RegisterType(screenShakeType);
            typeManager.RegisterType(serializationUtilType);
            typeManager.RegisterType(targetingUtilType);
            typeManager.RegisterType(teamUtilType);
            typeManager.RegisterType(textureUtilType);
            typeManager.RegisterType(timerUtilType);
            typeManager.RegisterType(concurrentSerializableDictionaryType);
            typeManager.RegisterType(serializableDictionaryType);
            typeManager.RegisterType(serializableListType);

            namespaceManager.RegisterNamespace(variableNamespace);
            typeManager.RegisterType(variableType);
            typeManager.RegisterType(computedVariableType);
            typeManager.RegisterType(computedVariableUtilType);
            typeManager.RegisterType(increaseType);
            typeManager.RegisterType(multiplierType);
            typeManager.RegisterType(temporaryIncreaseType);
            typeManager.RegisterType(temporaryMultiplierType);
            typeManager.RegisterType(boolVariableType);
            typeManager.RegisterType(floatVariableType);
            typeManager.RegisterType(floatComputedVariableType);
            typeManager.RegisterType(intVariableType);
            typeManager.RegisterType(intComputedVariableType);
            typeManager.RegisterType(stringVariableType);

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

        #region Coroutines
        public IEnumerator LoadSceneAsynchronously(string sceneIndex)
        {
            LoadingScreen loadingScreen = FindObjectOfType<LoadingScreen>();
            yield return loadingScreen.LoadSceneAsynchronously(sceneIndex);
        }
        #endregion
    }
}
