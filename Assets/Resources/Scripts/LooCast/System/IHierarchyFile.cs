using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchyFile : IHierarchyElement
    {
        #region Properties
        FilePath HierarchyFilePath { get; }

        IHierarchyFolder HierarchyFolderParent { get; }
        IEnumerable<IHierarchyObject> HierarchyObjectChildren { get; }
        #endregion
    }
}
