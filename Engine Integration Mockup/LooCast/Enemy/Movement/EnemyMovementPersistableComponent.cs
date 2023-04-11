using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Enemy.Movement
{
    [PersistableComponent]
    public abstract class EnemyMovementPersistableComponent : MovementPersistableComponent
    {
        [MetaDataInstance]
        public EnemyMovementMetaData MetaData;

        [PersistableDataInstance]
        public EnemyMovementPersistableData PersistableData;

        [LogicInstance]
        public EnemyMovementLogic Logic;
    } 
}
