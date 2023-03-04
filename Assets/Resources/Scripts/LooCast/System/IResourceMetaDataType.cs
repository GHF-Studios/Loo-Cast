using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IResourceMetaDataType : IObjectMetaDataType
    {
        #region Properties
        public IResourceMetaDataType ParentResourceMetaDataType { get; }
        public List<IResourceMetaDataType> ChildResourceMetaDataTypes { get; }
        #endregion
    }
}
