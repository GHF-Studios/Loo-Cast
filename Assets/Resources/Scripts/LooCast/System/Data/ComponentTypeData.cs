using LooCast.System.Types;
using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    public abstract class ComponentTypeData : InstanceTypeData, IComponentTypeData
    {
        #region Properties
        public abstract ComponentTypeData ParentComponentTypeData { get; }
        public abstract IEnumerable<ComponentTypeData> ChildComponentTypeData { get; }
        public abstract IPool<IComponentType.IComponent> ComponentPool { get; }
        #endregion
    }
}
