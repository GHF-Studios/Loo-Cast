namespace LooCast.System.Identification
{
    public interface IInstanceMetaDataIdentifier : IMetaDataIdentifier
    {
        #region Properties
        string InstanceMetaDataTypeID { get; }
        string InstanceMetaDataGUID { get; }
        string InstanceMetaDataID { get; }
        #endregion
    }
}