using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IInstanceMetaDataType : IMetaDataType
    {
        #region Properties
        public IInstanceMetaDataType ParentInstanceMetaDataType { get; }
        public List<IInstanceMetaDataType> ChildInstanceMetaDataTypes { get; }
        #endregion
    }
}
