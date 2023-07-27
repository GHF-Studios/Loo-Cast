using System.Collections.Generic;

namespace LooCast.System.Paths
{
    public interface IFolderPath : IPath
    {
        #region Properties
        List<string> FolderNames { get; }
        #endregion
    }
}
