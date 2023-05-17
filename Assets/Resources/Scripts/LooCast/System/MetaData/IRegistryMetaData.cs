using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    public interface IRegistryMetaData : IMetaData
    {
        #region Properties
        IRegistryMetaData RegistryMetaDataParent { get; }
        IEnumerable<IRegistryMetaData> RegistryMetaDataChildren { get; }
        #endregion
    }
}
