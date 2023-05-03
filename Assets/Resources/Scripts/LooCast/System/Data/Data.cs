using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    public abstract class Data : IData
    {
        #region Properties
        public abstract IData DataParent { get; }
        public abstract IEnumerable<IData> DataChildren { get; }

        public abstract ILooCastInstance Parent { get; }
        public abstract IEnumerable<ILooCastInstance> Children { get; }
        #endregion

        #region Fields
        public abstract bool Validate();
        #endregion
    }
}
