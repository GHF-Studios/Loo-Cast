using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface IResourceFileDataType : IResourceObjectDataType
    {
        #region Properties
        public IResourceFileDataType ParentResourceFileDataType { get; }
        public List<IResourceFileDataType> ChildResourceFileDataTypes { get; }
        #endregion
    }
}
