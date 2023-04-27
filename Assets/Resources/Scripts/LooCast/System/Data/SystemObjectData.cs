using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    public abstract class SystemObjectData : IInstanceData
    {
        #region Properties
        public abstract IInstanceData ParentInstanceData { get; }
        public abstract IEnumerable<IInstanceData> ChildInstanceData { get; }
        public abstract IData ParentData { get; }
        public abstract IEnumerable<IData> ChildData { get; }
        #endregion

        #region Fields
        public abstract bool Validate();
        #endregion
    }
}
