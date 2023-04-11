using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Enemy.Health
{
    [PersistableComponentType]
    public abstract class EnemyHealthPersistableComponentType<T> : HealthPersistableComponentType<T> where T : EnemyHealthPersistableComponent
    {
        [MetaDataTypeInstance]
        public EnemyHealthMetaDataType MetaDataType;

        [PersistableDataTypeInstance]
        public EnemyHealthPersistableDataType PersistableDataType;

        [LogicTypeInstance]
        public EnemyHealthLogicType LogicType;
    }
}