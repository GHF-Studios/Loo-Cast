using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchicalElement
    {
        #region Properties
        IHierarchicalElementPath HierarchicalElementPath { get; }
        HierarchicalElementType HierarchyElementType { get; }
        #endregion
    }
}
