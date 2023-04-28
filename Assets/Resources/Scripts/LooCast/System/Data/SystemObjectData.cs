using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Types;

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
