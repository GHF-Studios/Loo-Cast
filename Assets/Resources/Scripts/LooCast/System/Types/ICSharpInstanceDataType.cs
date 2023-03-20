using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface ICSharpInstanceDataType : IInstanceDataType
    {
        #region Properties
        public ICSharpInstanceDataType ParentCSharpInstanceDataType { get; }
        public List<ICSharpInstanceDataType> ChildCSharpInstanceDataTypes { get; }
        #endregion
    }
}
