namespace LooCast.System.Registries
{
    using LooCast.System.Identifiers;

    public sealed class NamespaceRegistry : Registry<INamespaceIdentifier, INamespace>
    {
        #region Constructors
        public NamespaceRegistry() : base(MainManager.Instance.MainRegistry)
        {

        }
        #endregion
    }
}
