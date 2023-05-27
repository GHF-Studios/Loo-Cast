using LooCast.System.Identifiers;
using System;
using System.Collections.Generic;

namespace LooCast.System.Hierarchies
{
    public abstract class Hierarchy<ElementType> : IHierarchyFolder, IHierarchy
        where ElementType : IHierarchyElement
    {
        #region Properties
        public IObjectIdentifier ObjectIdentifier => HierarchyIdentifier;
        public IHierarchyIdentifier HierarchyIdentifier => hierarchyIdentifier;

        public HierarchyElementPath HierarchyElementPath => HierarchyFolderPath;
        public HierarchyFolderPath HierarchyFolderPath => hierarchyFolderPath;

        public HierarchyElementType HierarchyElementType => HierarchyElementType.Folder;

        public IHierarchyElement<ElementType> RootElement => rootElement;

        public IHierarchy HierarchyParent => hierarchyParent;
        public IEnumerable<IHierarchy> HierarchyChildren => hierarchyChildren;

        public IHierarchyFolder FolderHierarchyParent => (IHierarchyFolder)HierarchyParent;
        public IEnumerable<IHierarchyFolder> FolderHierarchyChildren => (IEnumerable<IHierarchyFolder>)HierarchyChildren;
        public IEnumerable<IHierarchyFile> FileHierarchyChildren => null;
        #endregion

        #region Fields
        private HierarchyIdentifier hierarchyIdentifier;
        private HierarchyFolderPath hierarchyFolderPath;
        private IHierarchyElement<ElementType> rootElement;
        private IHierarchy hierarchyParent;
        private List<IHierarchy> hierarchyChildren;
        #endregion

        #region Constructors
        public Hierarchy(HierarchyIdentifier hierarchyIdentifier, HierarchyFolderPath hierarchyFolderPath, IHierarchyElement<ElementType> rootElement, IHierarchy hierarchyParent)
        {
            this.hierarchyIdentifier = hierarchyIdentifier;
            this.hierarchyFolderPath = hierarchyFolderPath;
            this.rootElement = rootElement;
            this.hierarchyParent = hierarchyParent;
            hierarchyChildren = new List<IHierarchy>();
        }
        #endregion

        #region Methods
        public void RegisterChild(IHierarchy child)
        {
            hierarchyChildren.Add(child);
        }

        public void UnregisterChild(IHierarchy child)
        {
            hierarchyChildren.Remove(child);
        }
        
        public void Add(IHierarchyElement element)
        {
            // Implement the method to use the root element to traverse the hierarchy and check if the element's path is valid not occupied by another element
            // if the path is valid, add the element to the hierarchy
            // if the path is not valid, throw an exception
        }

        public bool Remove(HierarchyElementPath path)
        {
            throw new NotImplementedException();
        }

        public void Clear()
        {
            throw new NotImplementedException();
        }

        public IHierarchyElement Get(HierarchyElementPath path)
        {
            throw new NotImplementedException();
        }

        public bool ContainsElement(IHierarchyElement element)
        {
            throw new NotImplementedException();
        }

        public bool ContainsPath(HierarchyElementPath path)
        {
            throw new NotImplementedException();
        }
        #endregion
    }
}
