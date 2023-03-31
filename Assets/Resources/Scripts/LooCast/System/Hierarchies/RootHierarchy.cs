using System;

namespace LooCast.System.Hierarchies
{
    public sealed class RootHierarchy : Hierarchy<Hierarchy<IHierarchyElement>>
    {
        public RootHierarchy(Hierarchy<IHierarchyElement> root, string hierarchyName, HierarchyPath hierarchyPath, Hierarchy<IHierarchyElement> parentHierarchy = null) : 
            base("LooCast.System.Hierarchies:RootHierarchy", root, hierarchyName, hierarchyPath, parentHierarchy)
        {
            
        }
    }
}
