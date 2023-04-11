using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Movement
{
    [PersistableComponent]
    public abstract class MovementPersistableComponent : PersistableComponent
    {
        [MetaDataInstance]
        public MovementMetaData MetaData;

        [PersistableDataInstance]
        public MovementPersistableData PersistableData;

        [LogicInstance]
        public MovementLogic Logic;
    } 
}
