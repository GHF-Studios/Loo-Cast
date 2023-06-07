using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchicalElement
    {
        #region Properties
        IHierarchicalElementPath HierarchicalElementPath { get; }
        HierarchyElementType HierarchyElementType { get; }
        #endregion
    }
}
