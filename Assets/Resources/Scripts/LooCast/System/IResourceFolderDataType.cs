using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IResourceFolderDataType : IResourceObjectDataType
    {
        #region Properties
        public IResourceFolderDataType ParentResourceFolderDataType { get; }
        public List<IResourceFolderDataType> ChildResourceFolderDataTypes { get; }
        #endregion
    }
}
