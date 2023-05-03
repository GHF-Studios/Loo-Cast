using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    public interface INamespaceData : IData
    {
        #region Properties
        public INamespaceData NamespaceDataParent { get; }
        public IEnumerable<INamespaceData> NamespaceDataChildren { get; }
        #endregion
    }
}
