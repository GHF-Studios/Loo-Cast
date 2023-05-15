using LooCast.System.Registries;
using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    public interface IRegistryMetaData : IMetaData
    {
        #region Properties
        IRegistryMetaData RegistryMetaDataParent { get; set; }
        IEnumerable<IRegistryMetaData> RegistryMetaDataChildren { get; set; }
        
        IRegistry RegistryParent { get; set; }
        IEnumerable<IRegistry> RegistryChildren { get; set; }
        #endregion
    }
}
