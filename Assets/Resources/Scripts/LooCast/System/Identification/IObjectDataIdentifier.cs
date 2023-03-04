namespace LooCast.System.Identification
{
    public interface IObjectDataIdentifier : ICSharpInstanceDataIdentifier
    {
        #region Properties
        string ObjectDataTypeID { get; }
        string ObjectDataGUID { get; }
        string ObjectDataID { get; }
        #endregion
    }
}