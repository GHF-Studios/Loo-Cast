using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Types;

    public interface ISystemObjectTypeData : IInstanceTypeData
    {
        #region Properties
        public ISystemObjectTypeData ParentSystemObjectTypeData { get; }
        public IEnumerable<ISystemObjectTypeData> ChildSystemObjectTypeData { get; }
        public IPool<ISystemObjectType.ISystemObject> SystemObjectPool { get; }
        #endregion
    }
}
