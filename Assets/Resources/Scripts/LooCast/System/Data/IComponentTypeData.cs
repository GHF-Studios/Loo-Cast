using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    using LooCast.System.Types;
    
    public interface IComponentTypeData : IInstanceTypeData
    {
        #region Properties
        public IComponentTypeData ParentComponentTypeData { get; }
        public IEnumerable<IComponentTypeData> ChildComponentTypeData { get; }
        public IPool<IComponentType.IComponent> ComponentPool { get; }
        #endregion
    }
}
