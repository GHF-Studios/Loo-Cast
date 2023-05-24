using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchy
    {
        #region Properties
        public IHierarchyIdentifier HierarchyIdentifier { get; }
        #endregion

        #region Methods
        public void Add(HierarchyElementPath path, HierarchyElement element);
        public bool Remove(HierarchyElementPath path);
        public HierarchyElement Get(HierarchyElementPath path);
        public bool ContainsPath(HierarchyElementPath path);
        public bool ContainsElement(HierarchyElement element);
        public void Clear();
        #endregion
    }
}
