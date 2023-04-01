using System.Collections.Generic;

namespace LooCast.System
{
    using global::LooCast.System.Registries;
    
    public interface IHierarchyElement : IIdentifiable
    {
        public HierarchyPath HierarchyPath { get; }
#nullable enable
        public IHierarchyElement? ParentElement { get; set; }
#nullable disable
        public List<IHierarchyElement> ChildElements { get; }
        public HierarchyElementRegistryRegistry SubHierarchies { get; }
        public HierarchyElementRegistryRegistry SuperHierarchies { get; }
    }
}
