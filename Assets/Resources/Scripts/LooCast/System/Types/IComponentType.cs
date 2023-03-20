using System;
using System.Collections.Generic;

namespace LooCast.System.Types
{
    public interface IComponentType : IGameObjectType
    {
        #region Properties
        public IComponentType ParentComponentType { get; }
        public List<IComponentType> ChildComponentTypes { get; }
        #endregion
    }
}
