using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Paths;

    public interface IFile : IHierarchicalElement, IChild<IFolder>, IParent<IObject>
    {
        #region Properties
        FilePath FilePath { get; }
        #endregion
    }
}
