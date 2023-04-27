using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    public interface ISystemObjectTypeMetaData : IInstanceTypeMetaData
    {
        #region Properties
        public ISystemObjectTypeMetaData ParentSystemObjectTypeMetaData { get; }
        public IEnumerable<ISystemObjectTypeMetaData> ChildSystemObjectTypesMetaData { get; }
        #endregion
    }
}
