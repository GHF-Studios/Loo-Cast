namespace LooCast.System.Identification
{
    public interface ICSharpInstanceMetaDataIdentifier : IInstanceMetaDataIdentifier
    {
        #region Properties
        string CSharpInstanceMetaDataTypeID { get; }
        string CSharpInstanceMetaDataGUID { get; }
        string CSharpInstanceMetaDataID { get; }
        #endregion
    }
}