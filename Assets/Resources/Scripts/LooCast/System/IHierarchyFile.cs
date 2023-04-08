using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchyFile : IHierarchyElement
    {
        #region Properties
        public HierarchyFilePath HierarchyFilePath { get; }
#nullable enable
        public IHierarchyFolder ParentHierarchyFolder { get; set; }
        public HashSet<IHierarchyObject>? ChildHierarchyObjects { get; }
#nullable disable
        #endregion
    }
}
