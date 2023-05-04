using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;
    
    public interface INamespaceMetaData : IMetaData
    {
        #region Properties
        public INamespaceIdentifier NamespaceIdentifier { get; }
        
        public INamespaceMetaData NamespaceMetaDataParent { get; }
        public IEnumerable<INamespaceMetaData> NamespaceMetaDataChildren { get; }
        public IEnumerable<ITypeMetaData> TypeMetaDataChildren { get; }

        public INamespace NamespaceParent { get; }
        public IEnumerable<INamespace> NamespaceChildren { get; }
        public IEnumerable<IType> TypeChildren { get; }
        #endregion
    }
}
