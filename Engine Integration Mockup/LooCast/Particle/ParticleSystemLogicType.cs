using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Particle
{
    [LogicType]
    public abstract class ParticleSystemLogicType<T> : LogicType<T> where T : ParticleSystemLogic
    {
        
    }
}