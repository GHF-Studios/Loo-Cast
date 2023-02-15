using System;
using UnityEngine;

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
    using Currency;
    using Data;
    using Diagnostic;
    using Enemy;
    using Event;
    using Experience;
    using Game;
    using Generator;
    using Health;
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
    using Scene;
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

    public class CoreManager : CoreModuleManager
    {
        #region Static Properties
        public static CoreManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[CoreManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = MainManager.Instance.transform;
                    return instanceObject.AddComponent<CoreManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static CoreManager instance;
        #endregion

        #region Properties
        #endregion

        #region Fields
        #endregion

        #region Callbacks

        #region Initialization Phases
        private void OnEarlyPreInitialize()
        {
            
        }

        private void OnPreInitialize()
        {
            
        }

        private void OnLatePreInitialize()
        {

        }

        private void OnEarlyInitialize()
        {

        }

        private void OnInitialize()
        {

        }

        private void OnLateInitialize()
        {

        }

        private void OnEarlyPostInitialize()
        {

        }

        private void OnPostInitialize()
        {

        }

        private void OnLatePostInitialize()
        {

        }
        #endregion

        #region Termination Phases
        private void OnEarlyPreTerminate()
        {

        }

        private void OnPreTerminate()
        {

        }

        private void OnLatePreTerminate()
        {

        }

        private void OnEarlyTerminate()
        {

        }

        private void OnTerminate()
        {

        }

        private void OnLateTerminate()
        {

        }

        private void OnEarlyPostTerminate()
        {

        }

        private void OnPostTerminate()
        {

        }

        private void OnLatePostTerminate()
        {

        }
        #endregion

        #endregion

        #region Methods
        public override void PreInitializeInstance()
        {
            base.PreInitializeInstance();

            #region Namespace/Type/Instance Registration
            NamespaceManager namespaceManager = NamespaceManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            InstanceManager instanceManager = InstanceManager.Instance;
            
            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast");
            looCastNamespace = new Namespace("Core", rootNamespace);
            looCastType = new Type(typeof(CoreManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);
            
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);
            #endregion
        }

        protected override ModuleManager[] GetModuleManagers()
        {
            return new ModuleManager[]
                {
                    // TODO: Sort by initialization order
                    AIManager.Instance,
                    AllyManager.Instance,
                    AOEManager.Instance,
                    ArcManager.Instance,
                    AsteroidManager.Instance,
                    AttributeManager.Instance,
                    StatManager.Instance,
                    BackgroundManager.Instance,
                    ChanceManager.Instance,
                    CurrencyManager.Instance,
                    DataManager.Instance,
                    DiagnosticManager.Instance,
                    EnemyManager.Instance,
                    EventManager.Instance,
                    ExperienceManager.Instance,
                    GameManager.Instance,
                    GeneratorManager.Instance,
                    HealthManager.Instance,
                    IndicatorManager.Instance,
                    InventoryManager.Instance,
                    ItemManager.Instance,
                    MainMenuManager.Instance,
                    MathManager.Instance,
                    MissionReceiver.Instance,
                    MovementManager.Instance,
                    NoiseManager.Instance,
                    ObserverManager.Instance,
                    OrbManager.Instance,
                    ParticleManager.Instance,
                    PlayerManager.Instance,
                    ProjectileManager.Instance,
                    RandomManager.Instance,
                    ResourceManager.Instance,
                    SceneManager.Instance,
                    SoundManager.Instance,
                    SpawnerManager.Instance,
                    StateMachineManager.Instance,
                    StationManager.Instance,
                    StatisticManager.Instance,
                    SteamworksManager.Instance,
                    TargetManager.Instance,
                    TestManager.Instance,
                    UIManager.Instance,
                    UniverseManager.Instance,
                    UtilManager.Instance,
                    VariableManager.Instance
                };
        }
        #endregion
    }
}