namespace LooCast.System.Identification
{
    public interface IResourceMetaDataIdentifier : IObjectMetaDataIdentifier
    {
        #region Properties
        string ResourceMetaDataTypeID { get; }
        string ResourceMetaDataGUID { get; }
        string ResourceMetaDataID { get; }
        #endregion
    }
}