using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface IUnityInstanceDataType : IInstanceDataType
    {
        #region Properties
        public IUnityInstanceDataType ParentUnityInstanceDataType { get; }
        public List<IUnityInstanceDataType> ChildUnityInstanceDataTypes { get; }
        #endregion
    }
}
