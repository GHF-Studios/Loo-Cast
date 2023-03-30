using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchyElement
    {
        public string HierarchyName { get; }
        public HierarchyPath HierarchyPath { get; }
#nullable enable
        public IHierarchyElement? Parent { get; set; }
#nullable disable
        public List<IHierarchyElement> Children { get; }
    }
}
