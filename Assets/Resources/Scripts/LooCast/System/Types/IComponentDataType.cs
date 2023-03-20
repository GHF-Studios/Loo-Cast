using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface IComponentDataType : IGameObjectDataType
    {
        #region Properties
        public IComponentDataType ParentComponentDataType { get; }
        public List<IComponentDataType> ChildComponentDataTypes { get; }
        #endregion
    }
}
