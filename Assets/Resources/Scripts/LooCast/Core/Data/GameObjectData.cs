using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Core.Data
{
    using LooCast.Core.Types;
    
    public abstract class GameObjectData : InstanceData, IGameObjectData
    {
        #region Properties
        public abstract IGameObjectData ParentGameObjectData { get; }
        public abstract IEnumerable<IGameObjectData> ChildGameObjectData { get; }
        public abstract IEnumerable<IComponentData> ChildComponentData { get; }
        
        public abstract IGameObjectType.IGameObject ParentGameObject { get; }
        public abstract IEnumerable<IGameObjectType.IGameObject> ChildGameObjects { get; }
        public abstract IEnumerable<IComponentType.IComponent> ChildComponents { get; }
        
        public abstract GameObject UnityEngineGameObject { get; }
        #endregion
    }
}
