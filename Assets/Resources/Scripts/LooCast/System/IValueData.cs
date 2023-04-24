using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IValueData
    {
        #region Properties
        public HashSet<IValueData> ChildValueData { get; }
        #endregion
    }
}
