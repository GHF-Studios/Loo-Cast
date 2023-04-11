using System;
using UnityEngine;
using LooCast.Enemy;
using LooCast.Health;
using LooCast.Movement;
using LooCast.Particle;
using LooCast.VFX;

namespace LooCast.Core
{
    [ModManager]
    public class LooCastModManager : ModManager
    {
        #region Module Managers
        
        [ModuleManagerInstance]
        public static EnemyModuleManager EnemyModuleManager;

        [ModuleManagerInstance]
        public static HealthModuleManager HealthModuleManager;

        [ModuleManagerInstance]
        public static MovementModuleManager MovementModuleManager;

        [ModuleManagerInstance]
        public static ParticleModuleManager ParticleModuleManager;

        [ModuleManagerInstance]
        public static VFXModuleManager VFXModuleManager;
        #endregion
    } 
}
