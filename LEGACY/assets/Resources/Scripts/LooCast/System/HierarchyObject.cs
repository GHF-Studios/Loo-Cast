using System.Collections.Generic;

namespace LooCast.System
{
    public class HierarchyObject : HierarchyElement
    {
        #region Properties
        public override HierarchyElementPath HierarchyElementPath => HierarchyObjectPath;
        public HierarchyObjectPath HierarchyObjectPath => hierarchyObjectPath;
#nullable enable
        public HierarchyFile? ParentHierarchyFile => parentHierarchyFile;
        public HierarchyObject? ParentHierarchyObject => parentHierarchyObject;
#nullable disable
        public IEnumerable<HierarchyObject> ChildHierarchyObjects => childHierarchyObjects;
        #endregion

        #region Fields
        private HierarchyObjectPath hierarchyObjectPath;
#nullable enable
        private HierarchyFile? parentHierarchyFile;
        private HierarchyObject? parentHierarchyObject;
#nullable disable
        private HashSet<HierarchyObject> childHierarchyObjects;
        #endregion

        #region Constructors
#nullable enable
        public HierarchyObject(HierarchyObjectPath hierarchyObjectPath, HierarchyFile parentHierarchyFile) : base(parentHierarchyFile)
        {
            this.hierarchyObjectPath = hierarchyObjectPath;
            this.parentHierarchyFile = parentHierarchyFile;
            parentHierarchyObject = null;
            childHierarchyObjects = new HashSet<HierarchyObject>();
        }

        public HierarchyObject(HierarchyObjectPath hierarchyObjectPath, HierarchyObject parentHierarchyObject) : base(parentHierarchyObject)
        {
            this.hierarchyObjectPath = hierarchyObjectPath;
            parentHierarchyFile = null;
            this.parentHierarchyObject = parentHierarchyObject;
            childHierarchyObjects = new HashSet<HierarchyObject>();
        }
#nullable disable
        #endregion

        #region Methods
        public bool AddChildHierarchyObject(HierarchyObject hierarchyObject)
        {
            return childHierarchyObjects.Add(hierarchyObject);
        }

        public bool RemoveChildHierarchyObject(HierarchyObject hierarchyObject)
        {
            return childHierarchyObjects.Remove(hierarchyObject);
        }
        #endregion
    }
}
