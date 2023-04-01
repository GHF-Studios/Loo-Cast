using System;

namespace LooCast.System.Hierarchies
{
    public sealed class MainHierarchy : Hierarchy<Hierarchy<IHierarchyElement>>
    {
        public MainHierarchy(Hierarchy<IHierarchyElement> root, string hierarchyName, HierarchyPath hierarchyPath, Hierarchy<IHierarchyElement> parentHierarchy = null) : 
            base("LooCast.System.Hierarchies:MainHierarchy", root, hierarchyName, hierarchyPath, parentHierarchy)
        {
            
        }
    }
}
