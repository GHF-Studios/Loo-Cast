using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.VFX
{
    [Entity]
    public abstract class Explosion : Entity
    {
        [DataInstance]
        public ExplosionData Data;

        [LogicComponentInstance]
        public ExplosionLogic Logic;
    } 
}
