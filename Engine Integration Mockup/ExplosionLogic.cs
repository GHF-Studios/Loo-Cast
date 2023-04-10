using System;
using UnityEngine;
using LooCast.Core;
using LooCast.Particle;

namespace LooCast.Enemy
{
    [Logic]
    public abstract class ExplosionLogic : Logic
    {
        [LogicInstance]
        public EnemyParticleSystemLogic ParticleSystemLogic;
    }
}