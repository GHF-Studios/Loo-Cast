using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Movement
{
    [PersistableComponentType]
    public abstract class MovementPersistableComponentType<T> : PersistableComponentType<T> where T : MovementPersistableComponent
    {
        [MetaDataTypeInstance]
        public MovementMetaDataType MetaDataType;

        [PersistableDataTypeInstance]
        public MovementPersistableDataType PersistableDataType;

        [LogicTypeInstance]
        public MovementLogicType LogicType;
    }
}