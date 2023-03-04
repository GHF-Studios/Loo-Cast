using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IObjectMetaDataType : ICSharpInstanceMetaDataType
    {
        #region Properties
        public IObjectMetaDataType ParentObjectMetaDataType { get; }
        public List<IObjectMetaDataType> ChildObjectMetaDataTypes { get; }
        #endregion
    }
}
