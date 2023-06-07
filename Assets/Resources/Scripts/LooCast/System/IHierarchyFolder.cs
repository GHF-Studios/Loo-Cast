using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchyFolder : IHierarchicalElement, IChild<IHierarchyFolder>, IParent<IHierarchyFolder>, IParent<IHierarchyFile>
    {
        #region Properties
        FolderPath HierarchyFolderPath { get; }
        #endregion
    }
}
