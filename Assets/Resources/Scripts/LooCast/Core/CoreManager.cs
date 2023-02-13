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

        #region Constructors
        public CoreManager() : base(GetDependencies(), GetSubModuleManagers(), GetModuleManagers())
        {
            
        }
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

        #region Static Methods
        /// <summary>
        /// Return the core module managers in no particular order.
        /// </summary>
        private static CoreModuleManager[] GetDependencies()
        {
            return new CoreModuleManager[]
                {
                    // ExampleMod.Core.CoreManager.Instance
                };
        }

        /// <summary>
        /// Return the sub-module managers in the order they should be initialized.
        /// </summary>
        private static SubModuleManager[] GetSubModuleManagers()
        {
            return new SubModuleManager[]
                {
                    // LooCast.Core.ExampleSubModule.ExampleSubModuleManager.Instance
                };
        }

        /// <summary>
        /// Return the module managers in the order they should be initialized.
        /// </summary>
        private static ModuleManager[] GetModuleManagers()
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
                    MissionManager.Instance,
                    MovementManager.Instance,
                    EffectManager.Instance,
                    NoiseManager.Instance,
                    ObserverManager.Instance,
                    OrbManager.Instance,
                    ParticleManager.Instance,
                    PlayerManager.Instance,
                    ProjectileManager.Instance,
                    RandomManager.Instance,
                    ResourceManager.Instance,
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