using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Paths;

    public interface IObject : IHierarchicalElement, IChild<IFile>, IChild<IObject>, IParent<IObject>
    {
        #region Properties
        ObjectPath ObjectPath{ get; }
        #endregion
    }
}
