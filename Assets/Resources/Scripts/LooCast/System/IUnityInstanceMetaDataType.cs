using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IUnityInstanceMetaDataType : IInstanceMetaDataType
    {
        #region Properties
        public IUnityInstanceMetaDataType ParentUnityInstanceMetaDataType { get; }
        public List<IUnityInstanceMetaDataType> ChildUnityInstanceMetaDataTypes { get; }
        #endregion
    }
}
