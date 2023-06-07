namespace LooCast.System.Paths
{
    public interface IFolderPath : IHierarchicalElementPath, IChild<FolderPath>
    {
        #region Properties
        FolderPath FolderPathParent { get; }
        #endregion
    }
}
