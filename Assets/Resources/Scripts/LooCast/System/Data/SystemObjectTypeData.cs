using LooCast.System.Types;
using System;
using System.Collections.Generic;

namespace LooCast.System.Data
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
