using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface IDataFileMetaDataType : IDataObjectMetaDataType
    {
        #region Properties
        public IDataFileMetaDataType ParentDataFileMetaDataType { get; }
        public List<IDataFileMetaDataType> ChildDataFileMetaDataTypes { get; }
        #endregion
    }
}
