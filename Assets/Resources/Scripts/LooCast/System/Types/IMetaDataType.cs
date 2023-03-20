using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface IMetaDataType : IObjectType
    {
        #region Properties
        public IMetaDataType ParentMetaDataType { get; }
        public List<IMetaDataType> ChildMetaDataTypes { get; }
        #endregion
    }
}
