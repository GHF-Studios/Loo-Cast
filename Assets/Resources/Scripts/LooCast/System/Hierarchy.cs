using System.Collections.Generic;

namespace LooCast.System
{
    public class Hierarchy<ElementType> : IHierarchyElement where ElementType : IHierarchyElement
    {
        #region Properties
        public string Name { get; }
        public ElementType RootElement { get; }
#nullable enable
        public Hierarchy<ElementType>? ParentHierarchy { get; }
        public HashSet<Hierarchy<ElementType>>? ChildHierarchies { get; }
#nullable disable
        public HierarchyElementPath HierarchyElementPath => RootElement.HierarchyElementPath;
        public string Id => RootElement.Id;
#nullable enable
        public IHierarchyElement? LocalHierarchyElementParent => ParentHierarchy;
        public HashSet<IHierarchyElement>? LocalHierarchyElementChildren => new HashSet<IHierarchyElement>(ChildHierarchies!);
        public HashSet<IHierarchyElement>? GlobalHierarchyElementParents => ParentHierarchy?.GlobalHierarchyElementParents;
        public HashSet<IHierarchyElement>? GlobalHierarchyElementChildren => new HashSet<IHierarchyElement>(ChildHierarchies!);
#nullable disable
        #endregion

        #region Constructors
        public Hierarchy(string name, ElementType rootElement, Hierarchy<ElementType>? parentHierarchy = null)
        {
            Name = name;
            RootElement = rootElement;
            ParentHierarchy = parentHierarchy;
            ChildHierarchies = new HashSet<Hierarchy<ElementType>>();
        }
        #endregion

        #region Methods
#nullable enable
        public bool TryRegisterElement(ElementType element)
        {
            if (TryGetElement(element.Id, out ElementType? existingElement))
            {
                if (existingElement.Equals(element))
                {
                    return true;
                }
                else
                {
                    return false;
                }
            }
            else
            {
                if (element.LocalHierarchyElementParent != null)
                {
                    element.LocalHierarchyElementParent.LocalHierarchyElementChildren!.Remove(element);
                }

                element.LocalHierarchyElementParent = this;
                RootElement.LocalHierarchyElementChildren!.Add(element);
                return true;
            }
        }

        public bool TryUnregisterElement(ElementType element)
        {
            if (TryGetElement(element.Id, out ElementType? existingElement))
            {
                if (existingElement.Equals(element))
                {
                    RootElement.LocalHierarchyElementChildren!.Remove(element);
                    element.LocalHierarchyElementParent = null;
                    return true;
                }
                else
                {
                    return false;
                }
            }
            else
            {
                return true;
            }
        }

        public bool TryGetElement(string id, out ElementType? element)
        {
            if (RootElement.Id == id)
            {
                element = RootElement;
                return true;
            }
            else
            {
                foreach (IHierarchyElement childElement in RootElement.LocalHierarchyElementChildren!)
                {
                    if (childElement.Id == id)
                    {
                        element = (ElementType)childElement;
                        return true;
                    }
                    else if (childElement is Hierarchy<ElementType> childHierarchy)
                    {
                        if (childHierarchy.TryGetElement(id, out element))
                        {
                            return true;
                        }
                    }
                }

                element = null;
                return false;
            }
        }
#nullable disable
        #endregion
    }
}
