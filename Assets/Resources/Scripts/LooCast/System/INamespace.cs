using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identifiers;
    using LooCast.System.Data;
    using LooCast.System.MetaData;
    
    public interface INamespace : ILooCastObject
    {
        #region Properties
        public INamespaceMetaData NamespaceMetaData { get; set; }
        public INamespaceData NamespaceData { get; }

        public INamespace NamespaceParent { get; }
        public IEnumerable<INamespace> NamespaceChildren { get; }
        public IEnumerable<IType> TypeChildren { get; }
        #endregion
    }
}
