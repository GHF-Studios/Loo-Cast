namespace LooCast
{
    public interface ITypeIdentifier : IGenericIdentifier<Type>
    {
        #region Properties
        System.Type SystemType { get; }
        #endregion
    }
}