using System.Collections.Generic;

namespace LooCast.System
{
    using global::LooCast.System.Registries;
    
    public interface IHierarchyElement : IIdentifiable
    {
        public string HierarchyName { get; }
        public HierarchyPath HierarchyPath { get; }
#nullable enable
        public Hierarchy<IHierarchyElement>? ParentHierarchy { get; set; }
#nullable disable
        public List<Hierarchy<IHierarchyElement>> ChildHierarchies { get; }
        public HierarchyElementRegistryRegistry SubHierarchies { get; }
        public HierarchyElementRegistryRegistry SuperHierarchies { get; }
    }
}
