using System.Collections.Generic;
using System.Linq;

namespace LooCast.System
{
    using global::LooCast.System.Identifiers;
    using global::LooCast.System.Managers;
    using global::LooCast.System.Registries;

    public abstract class Hierarchy<ElementType> : SystemObject, IHierarchyElement where ElementType : IHierarchyElement
    {
        #region Properties
        
        public ElementType Root => root;

        public string HierarchyName => hierarchyName;
        public HierarchyPath HierarchyPath => hierarchyPath;
        public Hierarchy<IHierarchyElement> ParentHierarchy
        {
            get
            {
                return parentHierarchy;
            }

            set
            {
                parentHierarchy = value;
            }
        }
        public List<Hierarchy<IHierarchyElement>> ChildHierarchies => childHierarchies;
        
        public HierarchyElementRegistryRegistry SubHierarchies => subHierarchies;
        public HierarchyElementRegistryRegistry SuperHierarchies => superHierarchies;

        #endregion

        #region Fields
        private readonly ElementType root;
        
        private readonly string hierarchyName;
        private readonly HierarchyPath hierarchyPath;
        private Hierarchy<IHierarchyElement> parentHierarchy;
        private List<Hierarchy<IHierarchyElement>> childHierarchies;

        private HierarchyElementRegistryRegistry subHierarchies;
        private HierarchyElementRegistryRegistry superHierarchies;
        #endregion

        #region Constructors
        public Hierarchy(TypeIdentifier typeIdentifier, ElementType root, string hierarchyName, HierarchyPath hierarchyPath, Hierarchy<IHierarchyElement> parentHierarchy = null) : base(typeIdentifier, parentHierarchy)
        {
            this.root = root;
            
            this.hierarchyName = hierarchyName;
            this.hierarchyPath = hierarchyPath;
            this.parentHierarchy = parentHierarchy;
            childHierarchies = new List<Hierarchy<IHierarchyElement>>();
        }
        #endregion

        #region Methods
#nullable enable
        public bool TryRegisterElement(ElementType element, ElementType? parentElement = default(ElementType?))
        {
            if (parentElement == null)
            {
                parentElement = root;
            }

            if (element == null)
            {
                return false;
            }

            if (element.ParentHierarchy != null)
            {
                return false;
            }

            if (parentElement.ChildHierarchies.Any(childElement => childElement.HierarchyName == element.HierarchyName))
            {
                return false;
            }

            parentElement.ChildHierarchies.Add(element);
            element.ParentHierarchy = parentElement;
            return true;
        }

        public bool TryUnregisterElement(ElementType element)
        {
            if (element == null)
            {
                return false;
            }

            if (element.ParentHierarchy == null)
            {
                return false;
            }

            if (!element.ParentHierarchy.ChildHierarchies.Remove(element))
            {
                return false;
            }

            element.ParentHierarchy = null;
            return true;
        }

        public bool TryGetElement(HierarchyPath elementHierarchyPath, out ElementType? element, ElementType? parentElement = default(ElementType?))
        {
            element = default(ElementType?);
            
            if (parentElement == null)
            {
                parentElement = root;
            }

            if (elementHierarchyPath == null)
            {
                return false;
            }

            if (elementHierarchyPath.IsRoot && elementHierarchyPath.PathSubStrings[0] == root.HierarchyName)
            {
                element = root;
                return true;
            }

            for (int i = 0; i < elementHierarchyPath.PathSubStrings.Length; i++)
            {
                string subElementName = elementHierarchyPath.PathSubStrings[i];

                ElementType? childElement = (ElementType?)parentElement.ChildHierarchies.FirstOrDefault(e => e.HierarchyName == subElementName);

                if (childElement == null)
                {
                    return false;
                }

                if (i == elementHierarchyPath.PathSubStrings.Length - 1)
                {
                    element = childElement;
                    return true;
                }

                parentElement = childElement;
            }
            
            return false;
        }
#nullable disable
        #endregion
    }
}
