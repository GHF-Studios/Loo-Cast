using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchyFolder : IHierarchyElement
    {
        #region Properties
        HierarchyFolderPath HierarchyFolderPath { get; }
        
#nullable enable
        IHierarchyFolder? FolderHierarchyParent { get; }
#nullable disable

        IEnumerable<IHierarchyFile> FileHierarchyChildren  { get; }
        IEnumerable<IHierarchyFolder> FolderHierarchyChildren  { get; }
        #endregion
    }
}
