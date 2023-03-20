using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface IResourceFileType : IResourceObjectType
    {
        #region Properties
        public IResourceFileType ParentResourceFileType { get; }
        public List<IResourceFileType> ChildResourceFileTypes { get; }
        #endregion
    }
}
