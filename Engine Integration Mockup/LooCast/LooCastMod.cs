using System;
using UnityEngine;
using LooCast.Enemy;
using LooCast.Health;
using LooCast.Movement;
using LooCast.Particle;
using LooCast.VFX;

namespace LooCast.Core
{
    [Mod]
    public class LooCastMod : Mod
    {
        #region Modules
        [ModuleInstance]
        public static EnemyModule EnemyModule;

        [ModuleInstance]
        public static HealthModule HealthModule;

        [ModuleInstance]
        public static MovementModule MovementModule;

        [ModuleInstance]
        public static ParticleModule ParticleModule;

        [ModuleInstance]
        public static VFXModule VFXModule;
        #endregion
    } 
}
