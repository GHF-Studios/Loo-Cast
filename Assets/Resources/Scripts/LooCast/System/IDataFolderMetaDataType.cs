﻿using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IDataFolderMetaDataType : IDataObjectMetaDataType
    {
        #region Properties
        public IDataFolderMetaDataType ParentDataFolderMetaDataType { get; }
        public List<IDataFolderMetaDataType> ChildDataFolderMetaDataTypes { get; }
        #endregion
    }
}
