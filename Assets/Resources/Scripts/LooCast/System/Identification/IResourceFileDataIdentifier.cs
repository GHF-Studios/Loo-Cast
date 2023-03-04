namespace LooCast.System.Identification
{
    public interface IResourceFileDataIdentifier : IResourceObjectDataIdentifier
    {
        #region Properties
        string ResourceFileDataTypeID { get; }
        string ResourceFileDataGUID { get; }
        string ResourceFileDataID { get; }
        #endregion
    }
}