using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Paths;
    
    public interface IFolder : IHierarchicalElement, IChild<IFolder>, IParent<IFolder>, IParent<IFile>
    {
        #region Properties
        FolderPath FolderPath { get; }
        #endregion
    }
}
