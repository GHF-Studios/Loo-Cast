using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchicalObject
    {
        #region Properties
        HierarchyElement ObjectHierarchyElement { get; }
        #endregion
    }
}
