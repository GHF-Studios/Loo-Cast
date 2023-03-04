using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IDataFileMetaDataType : IDataObjectMetaDataType
    {
        #region Properties
        public IDataFileMetaDataType ParentDataFileMetaDataType { get; }
        public List<IDataFileMetaDataType> ChildDataFileMetaDataTypes { get; }
        #endregion
    }
}
