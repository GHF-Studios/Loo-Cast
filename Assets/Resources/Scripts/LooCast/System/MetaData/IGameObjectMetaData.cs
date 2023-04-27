using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;
    
    public interface IGameObjectMetaData : IInstanceMetaData
    {
        #region Properties
        public abstract GameObject UnityEngineGameObject { get; }
        public IGameObjectIdentifier GameObjectIdentifier { get; }
        public IGameObjectTypeMetaData GameObjectTypeMetaData { get; }
        public IGameObjectMetaData ParentGameObjectMetaData { get; }
        public IEnumerable<IGameObjectMetaData> ChildGameObjectsMetaData { get; }
        public IEnumerable<IComponentMetaData> ChildComponentsMetaData { get; }
        #endregion
    }
}
