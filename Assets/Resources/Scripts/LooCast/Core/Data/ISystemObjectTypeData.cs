using System;
using System.Collections.Generic;

namespace LooCast.Core.Data
{
    using LooCast.Core.Types;

    public interface ISystemObjectTypeData : IInstanceTypeData
    {
        #region Properties
        public ISystemObjectTypeData ParentSystemObjectTypeData { get; }
        public IEnumerable<ISystemObjectTypeData> ChildSystemObjectTypeData { get; }
        public IPool<ISystemObjectType.ISystemObject> SystemObjectPool { get; }
        #endregion
    }
}
