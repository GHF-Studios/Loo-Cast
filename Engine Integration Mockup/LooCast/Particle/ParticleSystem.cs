using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Particle
{
    [Component]
    public abstract class Movement : Component
    {
        [MetaDataInstance]
        public EnemyMetaData MetaData;

        [PersistableDataInstance]
        public EnemyPersistableData PersistableData;

        [LogicComponentInstance]
        public EnemyLogic Logic;
    } 
}
