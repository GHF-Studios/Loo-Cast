using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface IGameObjectDataType : IUnityInstanceDataType
    {
        #region Properties
        public IGameObjectDataType ParentGameObjectDataType { get; }
        public List<IGameObjectDataType> ChildGameObjectDataTypes { get; }
        #endregion
    }
}
