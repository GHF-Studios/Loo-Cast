using LooCast.Core.Types;
using System;
using System.Collections.Generic;

namespace LooCast.Core.Data
{
    public abstract class SystemObjectTypeData : InstanceTypeData, ISystemObjectTypeData
    {
        #region Properties
        public abstract ISystemObjectTypeData ParentSystemObjectTypeData { get; }
        public abstract IEnumerable<ISystemObjectTypeData> ChildSystemObjectTypeData { get; }
        public abstract IPool<ISystemObjectType.ISystemObject> SystemObjectPool { get; }
        #endregion
    }
}
