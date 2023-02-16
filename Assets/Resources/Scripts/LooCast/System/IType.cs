namespace LooCast.System
{
    using System.Identification;
    
    public interface IType
    {
        #region Properties
        ITypeIdentifier TypeIdentifier { get; }
        #endregion
    }
}