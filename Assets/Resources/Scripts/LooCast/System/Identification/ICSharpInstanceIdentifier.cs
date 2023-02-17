namespace LooCast.System.Identification
{
    public interface ICSharpInstanceIdentifier : IInstanceIdentifier
    {
        #region Properties
        string CSharpInstanceTypeID { get; }
        string CSharpInstanceGUID { get; }
        string CSharpInstanceID { get; }
        #endregion
    }
}