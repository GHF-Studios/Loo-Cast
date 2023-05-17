using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    using LooCast.System.Identifiers;
    
    public interface IRegistryMetaData : IMetaData
    {
        #region Properties
        IRegistryIdentifier RegistryIdentifier { get; }
        
        IRegistryMetaData RegistryMetaDataParent { get; }
        #endregion
    }
}
