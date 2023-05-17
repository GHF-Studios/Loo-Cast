using System;
using System.Collections.Generic;

namespace LooCast.Core.Data
{
    using LooCast.Core.Types;

    public interface ISystemObjectData : IInstanceData
    {
        #region Properties
        public ISystemObjectData ParentSystemObjectData { get; }
        public IEnumerable<ISystemObjectData> ChildSystemObjectData { get; }

        public ISystemObjectType.ISystemObject ParentSystemObject { get; }
        public IEnumerable<ISystemObjectType.ISystemObject> ChildSystemObjects { get; }
        #endregion
    }
}
