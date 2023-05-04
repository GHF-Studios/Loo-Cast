using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    public interface IRegistryData : IData
    {
        #region Properties
        public IRegistryData RegistryDataParent { get; }
        public IEnumerable<IRegistryData> RegistryDataChildren { get; }

        public IEnumerable<ILooCastObject> Elements { get; }
        #endregion
    }
}
