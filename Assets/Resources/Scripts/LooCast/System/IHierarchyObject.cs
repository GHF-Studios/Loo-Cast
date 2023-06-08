using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Paths;

    public interface IHierarchyObject : IHierarchicalElement, IChild<IHierarchyFile>, IChild<IHierarchyObject>, IParent<IHierarchyObject>
    {
        #region Properties
        ObjectPath HierarchyObjectPath { get; }
        #endregion
    }
}
