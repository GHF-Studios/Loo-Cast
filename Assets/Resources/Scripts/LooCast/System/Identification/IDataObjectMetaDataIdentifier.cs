namespace LooCast.System.Identification
{
    public interface IDataObjectMetaDataIdentifier : IMetaDataIdentifier
    {
        #region Properties
        string DataObjectMetaDataTypeID { get; }
        string DataObjectMetaDataGUID { get; }
        string DataObjectMetaDataID { get; }
        #endregion
    }
}