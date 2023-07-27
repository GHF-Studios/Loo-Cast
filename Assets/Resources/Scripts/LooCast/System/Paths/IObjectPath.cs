using System.Collections.Generic;

namespace LooCast.System.Paths
{
    public interface IObjectPath : IPath
    {
        #region Properties
        List<string> ObjectNames { get; }
        FilePath FilePathParent { get; }
        #endregion
    }
}
