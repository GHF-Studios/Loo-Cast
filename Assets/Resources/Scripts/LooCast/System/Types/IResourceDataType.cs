using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface IResourceDataType : IObjectDataType
    {
        #region Properties
        public IResourceDataType ParentResourceDataType { get; }
        public List<IResourceDataType> ChildResourceDataTypes { get; }
        #endregion
    }
}
