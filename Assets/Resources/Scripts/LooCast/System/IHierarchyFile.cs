using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchyFile : IHierarchyElement
    {
        #region Properties
        HierarchyFilePath HierarchyFilePath { get; }
        
        IHierarchyFolder HierarchyFolderParent { get; }
        IEnumerable<IHierarchyObject> HierarchyObjectChildren { get; }
        #endregion
    }
}
