using System;
using System.Collections.Generic;

namespace LooCast.Core.Data
{
    using LooCast.Core.Types;

    public interface IGameObjectTypeData : IInstanceTypeData
    {
        #region Properties
        public IGameObjectTypeData ParentSystemObjectTypeData { get; }
        public IEnumerable<IGameObjectTypeData> ChildSystemObjectTypeData { get; }
        public IPool<IGameObjectType.IGameObject> GameObjectPool { get; }
        #endregion
    }
}
