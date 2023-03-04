namespace LooCast.System.Identification
{
    public interface IResourceFolderMetaDataIdentifier : IResourceObjectMetaDataIdentifier
    {
        #region Properties
        string ResourceFolderMetaDataTypeID { get; }
        string ResourceFolderMetaDataGUID { get; }
        string ResourceFolderMetaDataID { get; }
        #endregion
    }
}