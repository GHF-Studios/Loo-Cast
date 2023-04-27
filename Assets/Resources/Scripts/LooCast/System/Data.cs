using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public abstract class Data : IData
    {
        #region Properties
        public abstract IData ParentData { get; }
        public abstract IEnumerable<IData> ChildData { get; }
        #endregion

        #region Fields
        public abstract bool Validate();
        #endregion
    }
}
