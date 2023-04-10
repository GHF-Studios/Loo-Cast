using System;
using UnityEngine;

namespace LooCast.VFX
{
    [Data]
    public abstract class ExplosionData : Data
    {
        [DataInstance]
        public ExplosionParticleSystemData ParticleSystemData;
    }
}