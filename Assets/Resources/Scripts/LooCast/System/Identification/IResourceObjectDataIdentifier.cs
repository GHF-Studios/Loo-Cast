namespace LooCast.System.Identification
{
    public interface IResourceObjectDataIdentifier : IResourceDataIdentifier
    {
        #region Properties
        string ResourceObjectDataTypeID { get; }
        string ResourceObjectDataGUID { get; }
        string ResourceObjectDataID { get; }
        #endregion
    }
}