namespace LooCast.System.Identification
{
    public interface IResourceFolderIdentifier : IResourceObjectIdentifier
    {
        #region Properties
        string ResourceFolderTypeID { get; }
        string ResourceFolderGUID { get; }
        string ResourceFolderID { get; }
        #endregion
    }
}