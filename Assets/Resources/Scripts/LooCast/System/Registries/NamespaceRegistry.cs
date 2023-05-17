namespace LooCast.System.Registries
{
    using global::LooCast.System.Identifiers;

    public sealed class NamespaceRegistry : Registry<INamespaceIdentifier, INamespace>
    {
        #region Overides
        protected override IRegistry GetBaseRegistry()
        {
            return MainManager.Instance.MainRegistry;
        }
        #endregion
    }
}
