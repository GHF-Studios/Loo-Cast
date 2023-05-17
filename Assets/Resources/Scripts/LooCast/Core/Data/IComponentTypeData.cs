using System;
using System.Collections.Generic;

namespace LooCast.Core.Data
{
    using LooCast.Core.Types;
    
    public interface IComponentTypeData : IInstanceTypeData
    {
        #region Properties
        public IComponentTypeData ParentComponentTypeData { get; }
        public IEnumerable<IComponentTypeData> ChildComponentTypeData { get; }
        public IPool<IComponentType.IComponent> ComponentPool { get; }
        #endregion
    }
}
