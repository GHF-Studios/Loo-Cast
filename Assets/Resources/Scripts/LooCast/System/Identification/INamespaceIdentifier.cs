namespace LooCast.System.Identification
{
    public interface INamespaceIdentifier : IGenericIdentifier<Namespace>
    {
        #region Properties
        string ParentNamespaceID { get; }
        string Name { get; }
        string NamespaceID { get; }
        #endregion
    }
}