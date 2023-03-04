namespace LooCast.System.Identification
{
    public interface IObjectMetaDataIdentifier : ICSharpInstanceMetaDataIdentifier
    {
        #region Properties
        string ObjectMetaDataTypeID { get; }
        string ObjectMetaDataGUID { get; }
        string ObjectMetaDataID { get; }
        #endregion
    }
}