using System.Collections.Generic;

namespace LooCast.System
{
    public class HierarchyFile : HierarchyElement
    {
        #region Properties
        public override HierarchyElementPath HierarchyElementPath => HierarchyFilePath;
        public HierarchyFilePath HierarchyFilePath => hierarchyFilePath;
        public HierarchyFolder ParentHierarchyFolder => parentHierarchyFolder;
        public IEnumerable<HierarchyObject> ChildHierarchyObjects => childHierarchyObjects;
        #endregion

        #region Fields
        private HierarchyFilePath hierarchyFilePath;
        private HierarchyFolder parentHierarchyFolder;
        private HashSet<HierarchyObject> childHierarchyObjects;
        #endregion

        #region Constructors
#nullable enable
        public HierarchyFile(HierarchyFilePath hierarchyFilePath, HierarchyFolder parentHierarchyFolder) : base(parentHierarchyFolder)
        {
            if (parentHierarchyFolder == null)
            {
                throw new global::System.ArgumentNullException(nameof(parentHierarchyFolder));
            }
            else
            {
                parentHierarchyFolder.AddChildHierarchyFile(this);
            }

            this.hierarchyFilePath = hierarchyFilePath;
            this.parentHierarchyFolder = parentHierarchyFolder;
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
