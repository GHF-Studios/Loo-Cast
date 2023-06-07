using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchyObject : IHierarchyElement
    {
        #region Properties
        ObjectPath HierarchyObjectPath { get; }

#nullable enable
        IHierarchyFile? HierarchyFileParent { get; }
        IHierarchyObject? HierarchyObjectParent { get; }
#nullable disable
        
        IEnumerable<IHierarchyObject> HierarchyObjectChildren { get; }
        #endregion
    }
}
