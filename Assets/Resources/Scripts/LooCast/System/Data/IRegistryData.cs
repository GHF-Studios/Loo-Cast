using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    public interface IRegistryData : IData
    {
        #region Properties
        IRegistryData RegistryDataParent { get; set; }
        IEnumerable<IRegistryData> RegistryDataChildren { get; set; }
        #endregion
    }
}
