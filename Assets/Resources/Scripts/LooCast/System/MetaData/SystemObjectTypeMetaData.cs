using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;
    
    public abstract class SystemObjectTypeMetaData : InstanceTypeMetaData, ISystemObjectTypeMetaData
    {
        #region Properties
        public abstract ISystemObjectTypeMetaData ParentSystemObjectTypeMetaData { get; }
        public abstract IEnumerable<ISystemObjectTypeMetaData> ChildSystemObjectTypesMetaData { get; }
        #endregion
    }
}
