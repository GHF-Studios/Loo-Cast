using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Enemy
{
    [PersistableEntity]
    public abstract class Enemy : PersistableEntity
    {
        [MetaDataInstance]
        public EnemyMetaData MetaData;

        [PersistableDataInstance]
        public EnemyPersistableData PersistableData;

        [LogicComponentInstance]
        public EnemyLogic Logic;
    } 
}
