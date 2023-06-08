using UnityEngine;

namespace LooCast.System.Paths
{
    public interface IFilePath : IHierarchicalElementPath
    {
        #region Properties
        string FileName { get; }
        string FileExtension { get; }
        FolderPath FolderPathParent { get; }
        #endregion
    }
}
