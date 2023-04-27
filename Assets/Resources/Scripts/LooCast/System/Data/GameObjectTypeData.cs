using LooCast.System.Types;
using System;
using System.Collections.Generic;

namespace LooCast.System.Data
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
