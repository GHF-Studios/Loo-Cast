using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Paths;

    public interface IHierarchyFile : IHierarchicalElement, IChild<IHierarchyFolder>, IParent<IHierarchyObject>
    {
        #region Properties
        FilePath HierarchyFilePath { get; }
        #endregion
    }
}
