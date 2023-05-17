using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identifiers;
    using LooCast.System.Data;
    using LooCast.System.MetaData;
    
    public interface INamespace : ILooCastObject, IIdentifiable
    {
        #region Properties
        public INamespaceMetaData NamespaceMetaData { get; set; }
        public INamespaceData NamespaceData { get; set; }
        #endregion
    }
}
