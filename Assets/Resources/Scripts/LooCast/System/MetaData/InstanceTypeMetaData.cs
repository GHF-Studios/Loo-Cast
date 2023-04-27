using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    public abstract class InstanceTypeMetaData : TypeMetaData, IInstanceTypeMetaData
    {
        #region Properties
        public abstract IInstanceTypeMetaData ParentInstanceTypeMetaData { get; }
        public abstract IEnumerable<IInstanceTypeMetaData> ChildInstanceTypesMetaData { get; }
        #endregion
    }
}
