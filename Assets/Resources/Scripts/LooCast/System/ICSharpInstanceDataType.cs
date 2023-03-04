using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface ICSharpInstanceDataType : IInstanceDataType
    {
        #region Properties
        public ICSharpInstanceDataType ParentCSharpInstanceDataType { get; }
        public List<ICSharpInstanceDataType> ChildCSharpInstanceDataTypes { get; }
        #endregion
    }
}
