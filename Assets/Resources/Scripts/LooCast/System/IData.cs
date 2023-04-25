using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IData
    {
        #region Properties
#nullable enable
        public IData? ParentData { get; }
#nullable disable
        public HashSet<IData> ChildData { get; }
        #endregion

        #region Methods
        public bool Validate();
        #endregion
    }
}
