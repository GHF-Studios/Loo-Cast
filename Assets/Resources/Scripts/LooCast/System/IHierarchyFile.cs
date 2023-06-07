using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IHierarchyFile : IHierarchicalElement, IChild<IHierarchyFolder>, IParent<IHierarchyObject>
    {
        #region Properties
        FilePath HierarchyFilePath { get; }
        #endregion
    }
}
