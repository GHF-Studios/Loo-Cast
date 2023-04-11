using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Particle
{
    [ComponentType]
    public abstract class ParticleSystemComponentType<T> : ComponentType<T> where T : ParticleSystemComponent
    {
        [DataTypeInstance]
        public ParticleSystemDataType DataType;

        [LogicTypeInstance]
        public ParticleSystemLogicType LogicType;
    }
}