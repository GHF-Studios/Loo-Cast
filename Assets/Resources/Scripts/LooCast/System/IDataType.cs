using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IDataType : IObjectType
    {
        #region Properties
        public IDataType ParentDataType { get; }
        public List<IDataType> ChildDataTypes { get; }
        #endregion
    }
}
