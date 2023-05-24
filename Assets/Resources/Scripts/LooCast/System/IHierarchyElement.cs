using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchyElement
    {
        #region Properties
        HierarchyElementPath HierarchyElementPath { get; }
        #endregion
    }
}
