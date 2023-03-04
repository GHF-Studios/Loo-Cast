namespace LooCast.System.Identification
{
    public interface IResourceObjectMetaDataIdentifier : IResourceMetaDataIdentifier
    {
        #region Properties
        string ResourceObjectMetaDataTypeID { get; }
        string ResourceObjectMetaDataGUID { get; }
        string ResourceObjectMetaDataID { get; }
        #endregion
    }
}