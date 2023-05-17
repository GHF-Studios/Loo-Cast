using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.MetaData;
    
    public interface IRegistryData : IData
    {
        #region Properties
        IRegistryMetaData ContainingRegistryMetaData { get; }
        
        IRegistryData RegistryDataParent { get; }
        #endregion
    }
}
