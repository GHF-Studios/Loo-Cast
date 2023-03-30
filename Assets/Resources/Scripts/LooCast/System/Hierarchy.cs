using System.Collections.Generic;
using System.Linq;

namespace LooCast.System
{
    public abstract class Hierarchy<ElementType> : IHierarchyElement where ElementType : IHierarchyElement
    {
        #region Properties
        public ElementType Root => root;

        public string HierarchyName => hierarchyName;
        public HierarchyPath HierarchyPath => hierarchyPath;
        public IHierarchyElement Parent
        {
            get
            {
                return parentHierarchy;
            }

            set
            {
                parentHierarchy = (Hierarchy<IHierarchyElement>)value;
            }
        }
        public List<IHierarchyElement> Children => childHierarchies;
        #endregion

        #region Fields
        private readonly ElementType root;
        
        private readonly string hierarchyName;
        private readonly HierarchyPath hierarchyPath;
        private Hierarchy<IHierarchyElement> parentHierarchy;
        private List<IHierarchyElement> childHierarchies;
        #endregion

        #region Constructors
        public Hierarchy(ElementType root, string hierarchyName, HierarchyPath hierarchyPath, Hierarchy<IHierarchyElement> parentHierarchy = null)
        {
            this.root = root;
            this.hierarchyName = hierarchyName;
            this.hierarchyPath = hierarchyPath;
            this.parentHierarchy = parentHierarchy;
            childHierarchies = new List<IHierarchyElement>();
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

            if (element.Parent != null)
            {
                return false;
            }

            if (parentElement.Children.Any(childElement => childElement.HierarchyName == element.HierarchyName))
            {
                return false;
            }

            parentElement.Children.Add(element);
            element.Parent = parentElement;
            return true;
        }

        public bool TryUnregisterElement(ElementType element)
        {
            if (element == null)
            {
                return false;
            }

            if (element.Parent == null)
            {
                return false;
            }

            if (!element.Parent.Children.Remove(element))
            {
                return false;
            }

            element.Parent = null;
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

                ElementType? childElement = (ElementType?)parentElement.Children.FirstOrDefault(e => e.HierarchyName == subElementName);

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
