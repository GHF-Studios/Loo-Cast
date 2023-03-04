using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IDataObjectMetaDataType : IMetaDataType
    {
        #region Properties
        public IDataObjectMetaDataType ParentDataObjectMetaDataType { get; }
        public List<IDataObjectMetaDataType> ChildDataObjectMetaDataTypes { get; }
        #endregion
    }
}
