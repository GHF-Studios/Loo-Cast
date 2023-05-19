using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identifiers;
    using LooCast.System.Data;
    using LooCast.System.MetaData;
    
    public interface INamespace : IEngineObject
    {
        #region Properties
        public INamespaceIdentifier NamespaceIdentifier { get; }
        public INamespace NamespaceParent { get; }
        public IEnumerable<INamespace> NamespaceChildren { get; }
        public IEnumerable<IType> ContainedTypes { get; }
        #endregion
    }
}
