using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface ICSharpInstanceMetaDataType : IInstanceMetaDataType
    {
        #region Properties
        public ICSharpInstanceMetaDataType ParentCSharpInstanceMetaDataType { get; }
        public List<ICSharpInstanceMetaDataType> ChildCSharpInstanceMetaDataTypes { get; }
        #endregion
    }
}
