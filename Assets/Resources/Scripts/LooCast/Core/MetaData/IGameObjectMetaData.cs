using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Core.MetaData
{
    using LooCast.Core.Identifiers;
    using LooCast.Core.Types;

    public interface IGameObjectMetaData : IInstanceMetaData
    {
        #region Properties
        public IGameObjectIdentifier GameObjectIdentifier { get; }
        public IGameObjectTypeMetaData GameObjectTypeMetaData { get; }
        public IGameObjectMetaData ParentGameObjectMetaData { get; }
        public IEnumerable<IGameObjectMetaData> ChildGameObjectsMetaData { get; }
        public IEnumerable<IComponentMetaData> ChildComponentsMetaData { get; }

        public IGameObjectType GameObjectType { get; }
        public IGameObjectType.IGameObject ParentGameObject { get; }
        public IEnumerable<IGameObjectType.IGameObject> ChildGameObjects { get; }
        public IEnumerable<IComponentType.IComponent> ChildComponents { get; }
        #endregion
    }
}
