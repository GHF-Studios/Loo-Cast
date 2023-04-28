using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Types;
    
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
