using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Health
{
    [PersistableComponentType]
    public abstract class HealthPersistableComponentType<T> : PersistableComponentType<T> where T : HealthPersistableComponent
    {
        [MetaDataTypeInstance]
        public HealthMetaDataType MetaDataType;

        [PersistableDataTypeInstance]
        public HealthPersistableDataType PersistableDataType;

        [LogicTypeInstance]
        public HealthLogicType LogicType;
    }
}