using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    public interface IData
    {
        #region Properties
        public IData ParentData { get; }
        public IEnumerable<IData> ChildData { get; }

        public ILooCastInstance Parent { get; }
        public IEnumerable<ILooCastInstance> Children { get; }
        #endregion

        #region Methods
        public bool Validate();
        #endregion
    }
}
