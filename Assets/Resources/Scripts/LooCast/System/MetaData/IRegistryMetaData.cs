using LooCast.System.Registries;
using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    public interface IRegistryMetaData : IMetaData
    {
        #region Properties
        public IRegistryMetaData RegistryMetaDataParent { get; }
        public IEnumerable<IRegistryMetaData> RegistryMetaDataChildren { get; }
        
        public IRegistry RegistryParent { get; }
        public IEnumerable<IRegistry> RegistryChildren { get; }
        #endregion
    }
}
