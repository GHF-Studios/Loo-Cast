using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Types;

    public interface ITypeData : IData
    {
        #region Properties
        public ITypeData TypeDataParent { get; }
        public IEnumerable<ITypeData> TypeDataChildren { get; }
        #endregion
    }
}
