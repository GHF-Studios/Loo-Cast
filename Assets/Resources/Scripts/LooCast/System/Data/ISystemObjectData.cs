using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Types;

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
