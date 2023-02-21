namespace LooCast.System.Identification
{
    public interface INamespaceIdentifier : IIdentifier
    {
        #region Properties
        string ParentNamespaceID { get; }
        string Name { get; }
        string NamespaceID { get; }
        #endregion
    }
}