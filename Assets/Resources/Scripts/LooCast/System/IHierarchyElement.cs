using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identifiers;
    
    public interface IHierarchyElement : IIdentifiable
    {
        public string HierarchyName { get; }
        public HierarchyPath HierarchyPath { get; }
#nullable enable
        public IHierarchyElement? Parent { get; set; }
#nullable disable
        public List<IHierarchyElement> Children { get; }
        public Registry<TypeIdentifier, Registry<Identifier, IHierarchyElement>> SubHierarchies { get; }
    }
}
