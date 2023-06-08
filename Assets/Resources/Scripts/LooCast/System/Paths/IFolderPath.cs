using System.Collections.Generic;

namespace LooCast.System.Paths
{
    public interface IFolderPath : IHierarchicalElementPath
    {
        #region Properties
        List<string> FolderNames { get; }
        #endregion
    }
}
