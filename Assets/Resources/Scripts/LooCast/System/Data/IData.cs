using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    public interface IData
    {
        #region Properties
        public IData ParentData { get; }
        public IEnumerable<IData> ChildData { get; }

        public ILooCastObject Parent { get; }
        public IEnumerable<ILooCastObject> Children { get; }
        #endregion

        #region Methods
        public bool Validate();
        #endregion
    }
}
