using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface IInstanceDataType : IDataType
    {
        #region Properties
        public IInstanceDataType ParentInstanceDataType { get; }
        public List<IInstanceDataType> ChildInstanceDataTypes { get; }
        #endregion
    }
}
