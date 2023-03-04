namespace LooCast.System.Identification
{
    public interface ICSharpInstanceDataIdentifier : IInstanceDataIdentifier
    {
        #region Properties
        string CSharpInstanceDataTypeID { get; }
        string CSharpInstanceDataGUID { get; }
        string CSharpInstanceDataID { get; }
        #endregion
    }
}