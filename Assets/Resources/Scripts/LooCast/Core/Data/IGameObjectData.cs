using System;
using System.Collections.Generic;

namespace LooCast.Core.Data
{
    using LooCast.Core.Types;
    
    public interface IGameObjectData : IInstanceData
    {
        #region Properties
        public IGameObjectData ParentGameObjectData { get; }
        public IEnumerable<IGameObjectData> ChildGameObjectData { get; }
        public IEnumerable<IComponentData> ChildComponentData { get; }

        public IGameObjectType.IGameObject ParentGameObject { get; }
        public IEnumerable<IGameObjectType.IGameObject> ChildGameObjects { get; }
        public IEnumerable<IComponentType.IComponent> ChildComponents { get; }
        #endregion
    }
}
