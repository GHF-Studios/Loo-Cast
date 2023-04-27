using LooCast.System.Types;
using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    public abstract class InstanceTypeData : TypeData, IInstanceTypeData
    {
        #region Properties
        public abstract IInstanceTypeData ParentInstanceTypeData { get; }
        public abstract IEnumerable<IInstanceTypeData> ChildInstanceTypeData { get; }
        public abstract IPool<IInstanceType.IInstance> InstancePool { get; }
        #endregion
    }
}
