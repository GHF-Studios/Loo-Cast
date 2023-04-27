using LooCast.System.Types;
using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    public abstract class SystemObjectTypeData : InstanceTypeData, ISystemObjectTypeData
    {
        #region Properties
        public abstract SystemObjectTypeData ParentSystemObjectTypeData { get; }
        public abstract IEnumerable<SystemObjectTypeData> ChildSystemObjectTypeData { get; }
        public abstract IPool<ISystemObjectType.ISystemObject> SystemObjectPool { get; }
        #endregion
    }
}
