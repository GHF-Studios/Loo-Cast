using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;
    
    public interface IComponentMetaData : IInstanceMetaData
    {
        #region Properties
        public abstract Component UnityEngineComponent { get; }
        public IComponentIdentifier ComponentIdentifier { get; }
        public IComponentTypeMetaData ComponentTypeMetaData { get; }
        public abstract IGameObjectMetaData ParentGameObjectMetaData { get; }
        #endregion
    }
}
