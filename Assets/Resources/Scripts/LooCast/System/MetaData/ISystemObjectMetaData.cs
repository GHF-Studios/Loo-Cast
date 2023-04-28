using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;
    using LooCast.System.Types;
    
    public interface ISystemObjectMetaData : IInstanceMetaData
    {
        #region Properties
        public ISystemObjectIdentifier SystemObjectIdentifier { get; }
        public ISystemObjectTypeMetaData SystemObjectTypeMetaData { get; }
        public ISystemObjectMetaData ParentSystemObjectMetaData { get; }
        public IEnumerable<ISystemObjectMetaData> ChildSystemObjectsMetaData { get; }

        public ISystemObjectType SystemObjectType { get; }
        public ISystemObjectType.ISystemObject ParentSystemObject { get; }
        public IEnumerable<ISystemObjectType.ISystemObject> ChildSystemObjects { get; }
        #endregion
    }
}
