using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface ILooCastObject
    {
        #region Properties
        IIdentifier ObjectIdentifier { get; }
        HierarchyElement ObjectHierarchyElement { get; }
        #endregion
    }
}
