using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Types;

    public interface IInstanceData : IData
    {
        #region Properties
        public IInstanceData InstanceDataParent { get; }
        public IEnumerable<IInstanceData> InstanceDataChildren { get; }
        #endregion
    }
}
