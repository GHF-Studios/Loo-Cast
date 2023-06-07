namespace LooCast.System.Paths
{
    public interface IFilePath : IHierarchicalElementPath, IChild<FolderPath>
    {
        #region Properties
        FolderPath FolderPathParent { get; }
        #endregion
    }
}
