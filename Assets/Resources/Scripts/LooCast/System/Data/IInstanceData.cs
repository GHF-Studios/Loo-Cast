using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Types;
     
    public interface IInstanceData : IData
    {
        #region Properties
        public IInstanceData ParentInstanceData { get; }
        public IEnumerable<IInstanceData> ChildInstanceData { get; }

        public IInstanceType.IInstance ParentInstance { get; }
        public IEnumerable<IInstanceType.IInstance> ChildInstances { get; }
        #endregion
    }
}
