namespace LooCast
{
    public interface ITypeIdentifier : IGenericIdentifier<Type>
    {
        #region Properties
        string ParentNamespaceID { get; }
        string ParentTypeID { get; }
        System.Type SystemType { get; }
        string TypeID { get; }
        #endregion
    }
}