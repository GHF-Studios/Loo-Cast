using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchyObject : IHierarchicalElement, IChild<IHierarchyFile>, IChild<IHierarchyObject>, IParent<IHierarchyObject>
    {
        #region Properties
        ObjectPath HierarchyObjectPath { get; }
        #endregion
    }
}
