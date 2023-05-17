using System;
using System.Collections.Generic;

namespace LooCast.Core.Data
{
    using LooCast.Core.Types;
    
    public interface IComponentData : IInstanceData
    {
        #region Properties
        public IGameObjectData ParentGameObjectData { get; }

        public IGameObjectType.IGameObject ParentGameObject { get; }
        #endregion
    }
}
