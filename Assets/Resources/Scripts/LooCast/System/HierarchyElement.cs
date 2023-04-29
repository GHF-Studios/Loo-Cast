using System.Collections.Generic;

namespace LooCast.System
{
    public abstract class HierarchyElement : ILooCastInstance
    {
        #region Properties
        public IIdentifier Identifier => HierarchyElementPath;
        public abstract HierarchyElementPath HierarchyElementPath { get; }
#nullable enable
        public HierarchyElement? LocalHierarchyElementParent => localHierarchyElementParent;
#nullable disable
        public IEnumerable<HierarchyElement> LocalHierarchyElementChildren => localHierarchyElementChildren;
        public IEnumerable<HierarchyElement> GlobalHierarchyElementParents => globalHierarchyElementParents;
        public IEnumerable<HierarchyElement> GlobalHierarchyElementChildren => globalHierarchyElementChildren;
        #endregion

        #region Fields
#nullable enable
        private HierarchyElement? localHierarchyElementParent;
#nullable disable
        private HashSet<HierarchyElement> localHierarchyElementChildren;
        private HashSet<HierarchyElement> globalHierarchyElementParents;
        private HashSet<HierarchyElement> globalHierarchyElementChildren;
        #endregion

        #region Constructors
#nullable enable
        protected HierarchyElement(HierarchyElement? localHierarchyElementParent = null)
        {
            this.localHierarchyElementParent = localHierarchyElementParent;
            localHierarchyElementChildren = new HashSet<HierarchyElement>();
            globalHierarchyElementParents = new HashSet<HierarchyElement>();
            globalHierarchyElementChildren = new HashSet<HierarchyElement>();
        }
#nullable disable
        #endregion

        #region Methods
        public bool AddLocalHierarchyElementChild(HierarchyElement hierarchyElement)
        {
            return localHierarchyElementChildren.Add(hierarchyElement);
        }

        public bool RemoveLocalHierarchyElementChild(HierarchyElement hierarchyElement)
        {
            return localHierarchyElementChildren.Remove(hierarchyElement);
        }

        public bool AddGlobalHierarchyElementParent(HierarchyElement hierarchyElement)
        {
            return globalHierarchyElementParents.Add(hierarchyElement);
        }

        public bool RemoveGlobalHierarchyElementParent(HierarchyElement hierarchyElement)
        {
            return globalHierarchyElementParents.Remove(hierarchyElement);
        }

        public bool AddGlobalHierarchyElementChild(HierarchyElement hierarchyElement)
        {
            return globalHierarchyElementChildren.Add(hierarchyElement);
        }

        public bool RemoveGlobalHierarchyElementChild(HierarchyElement hierarchyElement)
        {
            return globalHierarchyElementChildren.Remove(hierarchyElement);
        }
        #endregion
    }
}
