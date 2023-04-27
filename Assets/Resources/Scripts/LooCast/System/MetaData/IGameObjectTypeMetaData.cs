using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    public interface IGameObjectTypeMetaData : IInstanceTypeMetaData
    {
        #region Properties
        public IGameObjectTypeMetaData ParentGameObjectTypeMetaData { get; }
        public IEnumerable<IGameObjectTypeMetaData> ChildGameObjectTypesMetaData { get; }
        public IEnumerable<IComponentTypeMetaData> ChildComponentTypesMetaData { get; }
        #endregion
    }
}
