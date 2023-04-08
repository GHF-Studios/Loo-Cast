using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchyElement : IIdentifiable
    {
        #region Properties
        public HierarchyElementPath HierarchyElementPath { get; }
#nullable enable
        public IHierarchyElement? LocalHierarchyElementParent { get; }
        public HashSet<IHierarchyElement>? LocalHierarchyElementChildren { get; }
        public HashSet<IHierarchyElement>? GlobalHierarchyElementParents { get; }
        public HashSet<IHierarchyElement>? GlobalHierarchyElementChildren { get; }
#nullable disable
        #endregion
    }
}
