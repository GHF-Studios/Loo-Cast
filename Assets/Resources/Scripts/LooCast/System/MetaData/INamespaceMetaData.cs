using LooCast.System.Data;
using LooCast.System.Identifiers;
using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    public interface INamespaceMetaData
    {
        #region Properties
        public INamespaceIdentifier NamespaceIdentifier { get; }
        
        public INamespaceMetaData NamespaceMetaDataParent { get; }
        public IEnumerable<INamespaceMetaData> NamespaceMetaDataChildren { get; }
        #endregion
    }
}
