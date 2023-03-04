namespace LooCast.System.Identification
{
    public interface IDataFileMetaDataIdentifier : IDataObjectMetaDataIdentifier
    {
        #region Properties
        string DataFileMetaDataTypeID { get; }
        string DataFileMetaDataGUID { get; }
        string DataFileMetaDataID { get; }
        #endregion
    }
}