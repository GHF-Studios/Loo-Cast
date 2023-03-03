using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IDataFolderType : IDataObjectType
    {
        #region Properties
        public IDataFolderType ParentDataFolderType { get; }
        public List<IDataFolderType> ChildDataFolderTypes { get; }
        #endregion
    }
}
