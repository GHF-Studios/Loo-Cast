using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface IResourceFolderType : IResourceObjectType
    {
        #region Properties
        public IResourceFolderType ParentResourceFolderType { get; }
        public List<IResourceFolderType> ChildResourceFolderTypes { get; }
        #endregion
    }
}
