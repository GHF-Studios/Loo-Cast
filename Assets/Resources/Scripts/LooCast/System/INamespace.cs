namespace LooCast.System
{
    using System.Identification;
    
    public interface INamespace
    {
        #region Properties
        INamespaceIdentifier NamespaceIdentifier { get; }
        #endregion
    }
}