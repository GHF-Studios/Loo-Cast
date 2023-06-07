namespace LooCast.System.Paths
{
    public interface IObjectPath : IHierarchicalElementPath, IChild<FilePath>, IChild<ObjectPath>
    {
        #region Properties
        FilePath FilePathParent { get; }
        ObjectPath ObjectPathParent { get; }
        #endregion
    }
}
