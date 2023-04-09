using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchyElement : IIdentifiable
    {
        #region Properties
        public HierarchyElementPath HierarchyElementPath { get; }
#nullable enable
        public IHierarchyElement LocalHierarchyElementParent { get; }
        public IEnumerable<IHierarchyElement> LocalHierarchyElementChildren { get; }
        public IEnumerable<IHierarchyElement> GlobalHierarchyElementParents { get; }
        public IEnumerable<IHierarchyElement> GlobalHierarchyElementChildren { get; }
#nullable disable
        #endregion
    }
}
