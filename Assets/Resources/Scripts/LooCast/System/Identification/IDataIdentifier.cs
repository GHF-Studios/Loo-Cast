namespace LooCast.System.Identification
{
    public interface IDataIdentifier
    {
        #region Properties
        ITypeIdentifier DataTypeIdentifier { get; }
        #endregion
    }
}