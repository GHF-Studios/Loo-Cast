using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;
    
    public abstract class GameObjectTypeMetaData : InstanceTypeMetaData, IGameObjectTypeMetaData
    {
        #region Properties
        public abstract IGameObjectTypeMetaData ParentGameObjectTypeMetaData { get; }
        public abstract IEnumerable<IGameObjectTypeMetaData> ChildGameObjectTypesMetaData { get; }
        public abstract IEnumerable<IComponentTypeMetaData> ChildComponentTypesMetaData { get; }
        #endregion
    }
}
