using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IMetaDataType : IObjectType
    {
        #region Properties
        public IMetaDataType ParentMetaDataType { get; }
        public List<IMetaDataType> ChildMetaDataTypes { get; }
        #endregion
    }
}
