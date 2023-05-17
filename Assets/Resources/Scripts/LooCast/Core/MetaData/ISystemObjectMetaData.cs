using System;
using System.Collections.Generic;

namespace LooCast.Core.MetaData
{
    using LooCast.Core.Identifiers;
    using LooCast.Core.Types;
    
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
