using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchyObject : IHierarchyElement
    {
        #region Properties
        public HierarchyObjectPath HierarchyObjectPath { get; }
#nullable enable
        public IHierarchyFile? ParentHierarchyFile { get; set; }
        public IHierarchyObject? ParentHierarchyObject { get; set; }
        public HashSet<IHierarchyObject>? ChildHierarchyObjects { get; }
#nullable disable
        #endregion
    }
}
