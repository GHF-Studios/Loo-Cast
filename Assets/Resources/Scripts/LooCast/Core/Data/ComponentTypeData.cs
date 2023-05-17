using LooCast.Core.Types;
using System;
using System.Collections.Generic;

namespace LooCast.Core.Data
{
    public abstract class ComponentTypeData : InstanceTypeData, IComponentTypeData
    {
        #region Properties
        public abstract IComponentTypeData ParentComponentTypeData { get; }
        public abstract IEnumerable<IComponentTypeData> ChildComponentTypeData { get; }
        public abstract IPool<IComponentType.IComponent> ComponentPool { get; }
        #endregion
    }
}
