using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Movement
{
    [PersistableComponent]
    public abstract class Movement : PersistableComponent
    {
        [MetaDataInstance]
        public EnemyMetaData MetaData;

        [PersistableDataInstance]
        public EnemyPersistableData PersistableData;

        [LogicComponentInstance]
        public EnemyLogic Logic;
    } 
}
