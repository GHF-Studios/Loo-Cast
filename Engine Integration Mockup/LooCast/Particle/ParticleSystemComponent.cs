using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Particle
{
    [Component]
    public abstract class ParticleSystemComponent : Component
    {
        [DataInstance]
        public ParticleSystemData Data;

        [LogicInstance]
        public ParticleSystemLogic Logic;
    } 
}
