using System.Collections.Generic;

namespace LooCast.System
{
    public class HierarchyFolder : HierarchyElement
    {
        #region Properties
        public override HierarchyElementPath HierarchyElementPath => HierarchyFolderPath;
        public HierarchyFolderPath HierarchyFolderPath => hierarchyFolderPath;
#nullable enable
        public HierarchyFolder? ParentHierarchyFolder => parentHierarchyFolder;
#nullable disable
        public IEnumerable<HierarchyFile> ChildHierarchyFiles => childHierarchyFiles;
        public IEnumerable<HierarchyFolder> ChildHierarchyFolders => childHierarchyFolders;
        #endregion

        #region Fields
        private HierarchyFolderPath hierarchyFolderPath;
#nullable enable
        private HierarchyFolder? parentHierarchyFolder;
#nullable disable
        private HashSet<HierarchyFile> childHierarchyFiles;
        private HashSet<HierarchyFolder> childHierarchyFolders;
        #endregion

        #region Constructors
#nullable enable
        public HierarchyFolder(HierarchyFolderPath hierarchyFolderPath, HierarchyFolder? parentHierarchyFolder = null) : base(parentHierarchyFolder)
        {
            if (parentHierarchyFolder != null)
            {
                parentHierarchyFolder.AddChildHierarchyFolder(this);
            }   

            this.hierarchyFolderPath = hierarchyFolderPath;
            this.parentHierarchyFolder = parentHierarchyFolder;
            childHierarchyFiles = new HashSet<HierarchyFile>();
            childHierarchyFolders = new HashSet<HierarchyFolder>();
        }
#nullable disable
        #endregion

        #region Methods
        public bool AddChildHierarchyFile(HierarchyFile hierarchyFile)
        {
            return childHierarchyFiles.Add(hierarchyFile);
        }

        public bool RemoveChildHierarchyFile(HierarchyFile hierarchyFile)
        {
            return childHierarchyFiles.Remove(hierarchyFile);
        }

        public bool AddChildHierarchyFolder(HierarchyFolder hierarchyFolder)
        {
            return childHierarchyFolders.Add(hierarchyFolder);
        }

        public bool RemoveChildHierarchyFolder(HierarchyFolder hierarchyFolder)
        {
            return childHierarchyFolders.Remove(hierarchyFolder);
        }
        #endregion
    }
}
