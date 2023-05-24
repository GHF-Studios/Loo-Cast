using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchyObject : IHierarchyElement
    {
        #region Properties
        new public HierarchyElementPath HierarchyElementPath => HierarchyObjectPath;
        HierarchyObjectPath HierarchyObjectPath { get; }
        #endregion
    }
}
