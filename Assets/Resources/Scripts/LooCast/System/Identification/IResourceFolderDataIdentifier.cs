namespace LooCast.System.Identification
{
    public interface IResourceFolderDataIdentifier : IResourceObjectDataIdentifier
    {
        #region Properties
        string ResourceFolderDataTypeID { get; }
        string ResourceFolderDataGUID { get; }
        string ResourceFolderDataID { get; }
        #endregion
    }
}