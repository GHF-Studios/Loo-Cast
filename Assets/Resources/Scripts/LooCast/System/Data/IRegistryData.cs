using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    public interface IRegistryData : IData
    {
        #region Properties
        public IEnumerable<ILooCastObject> Elements { get; }
        
        public IRegistryData RegistryDataParent { get; }
        public IEnumerable<IRegistryData> RegistryDataChildren { get; }
        #endregion
    }
}
