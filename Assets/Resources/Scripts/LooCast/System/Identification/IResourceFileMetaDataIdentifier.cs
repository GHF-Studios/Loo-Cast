namespace LooCast.System.Identification
{
    public interface IResourceFileMetaDataIdentifier : IResourceObjectMetaDataIdentifier
    {
        #region Properties
        string ResourceFileMetaDataTypeID { get; }
        string ResourceFileMetaDataGUID { get; }
        string ResourceFileMetaDataID { get; }
        #endregion
    }
}