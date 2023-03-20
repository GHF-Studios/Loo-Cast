using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface IResourceObjectDataType : IResourceDataType
    {
        #region Properties
        public IResourceObjectDataType ParentResourceObjectDataType { get; }
        public List<IResourceObjectDataType> ChildResourceObjectDataTypes { get; }
        #endregion
    }
}
