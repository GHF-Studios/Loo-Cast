using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchyFolder : IHierarchyElement
    {
        #region Properties
        new public HierarchyElementPath HierarchyElementPath => HierarchyFolderPath;
        HierarchyFolderPath HierarchyFolderPath { get; }
        #endregion
    }
}
