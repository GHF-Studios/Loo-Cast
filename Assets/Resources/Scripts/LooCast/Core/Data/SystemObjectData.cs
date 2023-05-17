using System;
using System.Collections.Generic;

namespace LooCast.Core.Data
{
    using LooCast.Core.Types;

    public abstract class SystemObjectData : InstanceData, ISystemObjectData
    {
        #region Properties
        public abstract ISystemObjectData ParentSystemObjectData { get; }
        public abstract IEnumerable<ISystemObjectData> ChildSystemObjectData { get; }

        public abstract ISystemObjectType.ISystemObject ParentSystemObject { get; }
        public abstract IEnumerable<ISystemObjectType.ISystemObject> ChildSystemObjects { get; }
        #endregion
    }
}
