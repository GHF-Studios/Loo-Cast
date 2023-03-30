using System;

namespace LooCast.System.Hierarchies
{
    public class RootHierarchy : Hierarchy<Hierarchy<IHierarchyElement>>
    {
        public RootHierarchy(Hierarchy<IHierarchyElement> root, string hierarchyName, HierarchyPath hierarchyPath, Hierarchy<IHierarchyElement> parentHierarchy = null) : base(root, hierarchyName, hierarchyPath, parentHierarchy)
        {
            
        }
    }
}
