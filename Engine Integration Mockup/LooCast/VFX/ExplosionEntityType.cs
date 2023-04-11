using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.VFX
{
    [EntityType]
    public abstract class ExplosionEntityType<T> : EntityType<T> where T : ExplosionEntity
    {
        [ComponentTypeInstance]
        public ExplosionParticleSystemComponentType ParticleSystemComponentType;
    }
}