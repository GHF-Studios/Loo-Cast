using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IUnityInstanceDataType : IInstanceDataType
    {
        #region Properties
        public IUnityInstanceDataType ParentUnityInstanceDataType { get; }
        public List<IUnityInstanceDataType> ChildUnityInstanceDataTypes { get; }
        #endregion
    }
}
