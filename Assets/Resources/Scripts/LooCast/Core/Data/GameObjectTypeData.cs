using LooCast.Core.Types;
using System;
using System.Collections.Generic;

namespace LooCast.Core.Data
{
    public abstract class GameObjectTypeData : InstanceTypeData, IGameObjectTypeData
    {
        #region Properties
        public abstract IGameObjectTypeData ParentSystemObjectTypeData { get; }
        public abstract IEnumerable<IGameObjectTypeData> ChildSystemObjectTypeData { get; }
        public abstract IPool<IGameObjectType.IGameObject> GameObjectPool { get; }
        #endregion
    }
}
