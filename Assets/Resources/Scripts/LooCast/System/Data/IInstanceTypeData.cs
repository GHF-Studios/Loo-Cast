using LooCast.System.Types;
using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    public interface IInstanceTypeData : ITypeData
    {
        #region Properties
        public IInstanceTypeData ParentInstanceTypeData { get; }
        public IEnumerable<IInstanceTypeData> ChildInstanceTypeData { get; }

        public IPool<IInstanceType.IInstance> InstancePool { get; }
        #endregion
    }
}
