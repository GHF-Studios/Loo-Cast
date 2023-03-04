﻿using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IResourceFileMetaDataType : IResourceObjectMetaDataType
    {
        #region Properties
        public IResourceFileMetaDataType ParentResourceFileMetaDataType { get; }
        public List<IResourceFileMetaDataType> ChildResourceFileMetaDataTypes { get; }
        #endregion
    }
}
