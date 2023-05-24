using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchyFile : IHierarchyElement
    {
        #region Properties
        new public HierarchyElementPath HierarchyElementPath => HierarchyFilePath;
        HierarchyFilePath HierarchyFilePath { get; }
        #endregion
    }
}
