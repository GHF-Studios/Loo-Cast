using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    public interface IRegistryData : IData
    {
        #region Properties
        IRegistryData RegistryDataParent { get; }
        IEnumerable<IRegistryData> RegistryDataChildren { get; }
        #endregion
    }
}
