using System;
using System.Collections.Generic;

namespace LooCast.System.Hierarchies
{
    using LooCast.System.Identifiers;

    public interface IHierarchy : IIdentifiableObject, IHierarchyElement
    {
        #region Properties
        public IHierarchyIdentifier HierarchyIdentifier { get; }
        #endregion

        #region Methods
        public void Add(IHierarchyElement element);
        public bool Remove(HierarchyElementPath path);
        public void Clear();
        public IHierarchyElement Get(HierarchyElementPath path);
        public bool ContainsElement(IHierarchyElement element);
        public bool ContainsPath(HierarchyElementPath path);
        #endregion
    }
}
