using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Types;

    public interface IGameObjectTypeData : IInstanceTypeData
    {
        #region Properties
        public IGameObjectTypeData ParentSystemObjectTypeData { get; }
        public IEnumerable<IGameObjectTypeData> ChildSystemObjectTypeData { get; }
        public IPool<IGameObjectType.IGameObject> GameObjectPool { get; }
        #endregion
    }
}
