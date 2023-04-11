using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Enemy.Movement
{
    [PersistableComponentType]
    public abstract class EnemyMovementPersistableComponentType<T> : MovementPersistableComponentType<T> where T : EnemyMovementPersistableComponent
    {
        [MetaDataTypeInstance]
        public EnemyMovementMetaDataType MetaDataType;

        [PersistableDataTypeInstance]
        public EnemyMovementPersistableDataType PersistableDataType;

        [LogicTypeInstance]
        public EnemyMovementLogicType LogicType;
    }
}