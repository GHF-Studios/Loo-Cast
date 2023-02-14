namespace LooCast
{
    public interface ITypeIdentifier : IGenericIdentifier<Namespace>
    {
        #region Properties
        System.Type SystemType { get; }
        #endregion
    }
}