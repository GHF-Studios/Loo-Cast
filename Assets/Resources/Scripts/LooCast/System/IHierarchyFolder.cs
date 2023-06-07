using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchyFolder : IHierarchicalElement
    {
        #region Properties
        FolderPath HierarchyFolderPath { get; }

#nullable enable
        IHierarchyFolder? FolderHierarchyParent { get; }
#nullable disable

        IEnumerable<IHierarchyFile> FileHierarchyChildren  { get; }
        IEnumerable<IHierarchyFolder> FolderHierarchyChildren  { get; }
        #endregion
    }
}
