using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchyElement
    {
        #region Properties
        HierarchyElementPath HierarchyElementPath { get; }
        HierarchyElementType HierarchyElementType { get; }
        #endregion
    }

    public interface IHierarchyElement<T> : IHierarchyElement
        where T : IHierarchyElement
    {
        #region Properties
        T HierarchyParent { get; }
        IEnumerable<T> HierarchyChildren { get; }
        #endregion

        #region Methods
        void RegisterChild(T child);
        void UnregisterChild(T child);
        #endregion
    }
}
