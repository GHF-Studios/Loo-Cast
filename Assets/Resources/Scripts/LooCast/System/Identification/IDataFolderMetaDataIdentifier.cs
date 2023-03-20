namespace LooCast.System.Identification
{
    public interface IDataFolderMetaDataIdentifier : IDataObjectMetaDataIdentifier
    {
        #region Properties
        string DataFolderMetaDataTypeID { get; }
        string DataFolderMetaDataGUID { get; }
        string DataFolderMetaDataID { get; }
        #endregion
    }
}