using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface IResourceFolderMetaDataType : IResourceObjectMetaDataType
    {
        #region Properties
        public IResourceFolderMetaDataType ParentResourceFolderMetaDataType { get; }
        public List<IResourceFolderMetaDataType> ChildResourceFolderMetaDataTypes { get; }
        #endregion
    }
}
