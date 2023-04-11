using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Health
{
    [PersistableComponent]
    public abstract class HealthPersistableComponent : PersistableComponent
    {
        [MetaDataInstance]
        public HealthMetaData MetaData;

        [PersistableDataInstance]
        public HealthPersistableData PersistableData;

        [LogicInstance]
        public HealthLogic Logic;
    } 
}
