namespace LooCast.System.Identification
{
    public interface IMetaDataIdentifier : IObjectIdentifier
    {
        #region Properties
        string MetaDataTypeID { get; }
        string MetaDataGUID { get; }
        string MetaDataID { get; }
        #endregion
    }
}