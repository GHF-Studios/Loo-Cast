using System;
using UnityEngine;
using LooCast.Core;
using LooCast.Particle;

namespace LooCast.VFX
{
    [Entity]
    public abstract class ExplosionEntity : Entity
    {
        [ComponentInstance]
        public ParticleSystemComponent ParticleSystemComponent;
    } 
}
