using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;

    [Serializable]
    public abstract class GameObjectMetaData : InstanceMetaData, IGameObjectMetaData
    {
        #region Properties
        public abstract GameObject UnityEngineGameObject { get; }
        public abstract IGameObjectIdentifier GameObjectIdentifier { get; }
        public abstract IGameObjectTypeMetaData GameObjectTypeMetaData { get; }
        public abstract IGameObjectMetaData ParentGameObjectMetaData { get; }
        public abstract IEnumerable<IGameObjectMetaData> ChildGameObjectsMetaData { get; }
        public abstract IEnumerable<IComponentMetaData> ChildComponentsMetaData { get; }
        #endregion
    }
}
