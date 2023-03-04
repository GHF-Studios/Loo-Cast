namespace LooCast.System.Identification
{
    public interface IResourceDataIdentifier : IObjectDataIdentifier
    {
        #region Properties
        string ResourceDataTypeID { get; }
        string ResourceDataGUID { get; }
        string ResourceDataID { get; }
        #endregion
    }
}