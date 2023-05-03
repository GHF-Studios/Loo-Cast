using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;
    using LooCast.System.Types;

    public interface IComponentMetaData : IInstanceMetaData
    {
        #region Properties
        public IComponentIdentifier ComponentIdentifier { get; }
        
        public IComponentTypeMetaData ComponentTypeMetaData { get; }
        public IGameObjectMetaData ParentGameObjectMetaData { get; }

        public IComponentType ComponentType { get; }
        public IGameObjectType.IGameObject ParentGameObject { get; }
        #endregion
    }
}
