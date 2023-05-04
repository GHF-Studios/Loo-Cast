using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;
    
    public interface ITypeMetaData : IMetaData
    {
        #region Properties
        public ITypeIdentifier TypeIdentifier { get; }

        public IEnumerable<IInstanceMetaData> TypeInstancesMetaData { get; }

        public ITypeMetaData TypeMetaDataParent { get; }
        public IEnumerable<ITypeMetaData> TypeMetaDataChildren { get; }

        public IEnumerable<IInstance> TypeInstances { get; }
        
        public IType TypeParent { get; }
        public IEnumerable<IType> TypeChildren { get; }
        #endregion
    }
}
