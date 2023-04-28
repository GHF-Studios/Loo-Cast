using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Types;
    
    public interface IComponentData : IInstanceData
    {
        #region Properties
        public IGameObjectData ParentGameObjectData { get; }

        public IGameObjectType.IGameObject ParentGameObject { get; }
        #endregion
    }
}
