using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    public interface IInstanceData : IData
    {
        #region Properties
        public IInstanceData ParentInstanceData { get; }
        public IEnumerable<IInstanceData> ChildInstanceData { get; }
        #endregion
    }
}
