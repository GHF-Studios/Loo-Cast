using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchicalElement
    {
        #region Properties
        HierarchicalObjectPath HierarchicalObjectPath { get; }
        HierarchyElementType HierarchyElementType { get; }
        #endregion
    }
}
