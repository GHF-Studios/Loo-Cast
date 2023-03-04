using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IResourceObjectMetaDataType : IResourceMetaDataType
    {
        #region Properties
        public IResourceObjectMetaDataType ParentResourceObjectMetaDataType { get; }
        public List<IResourceObjectMetaDataType> ChildResourceObjectMetaDataTypes { get; }
        #endregion
    }
}
