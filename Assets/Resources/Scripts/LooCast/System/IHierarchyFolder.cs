using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchyFolder : IHierarchyElement
    {
        #region Properties
        public HierarchyFolderPath HierarchyFolderPath { get; }
#nullable enable
        public IHierarchyFolder? ParentHierarchyFolder { get; set; }
        public HashSet<IHierarchyFile>? ChildHierarchyFiles { get; }
        public HashSet<IHierarchyFolder>? ChildHierarchyFolders { get; }
#nullable disable
        #endregion
    }
}
