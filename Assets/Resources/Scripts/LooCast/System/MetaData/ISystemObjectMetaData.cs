using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;
    
    public interface ISystemObjectMetaData : IInstanceMetaData
    {
        #region Properties
        public ISystemObjectIdentifier SystemObjectIdentifier { get; }
        public ISystemObjectTypeMetaData SystemObjectTypeMetaData { get; }
        public ISystemObjectMetaData ParentSystemObjectMetaData { get; }
        public IEnumerable<ISystemObjectMetaData> ChildSystemObjectsMetaData { get; }
        #endregion
    }
}
