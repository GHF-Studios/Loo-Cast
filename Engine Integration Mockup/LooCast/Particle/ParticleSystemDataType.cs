using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Particle
{
    [DataType]
    public abstract class ParticleSystemDataType<T> : DataType<T> where T : ParticleSystemData
    {
        
    }
}