namespace LooCast.System.Registries
{
    using global::LooCast.System.Identifiers;

    public sealed class MetaDataRegistry : Registry<IRegistryIdentifier, IRegistry>
    {
        #region Overides
        protected override IRegistry GetBaseRegistry()
        {
            return MainManager.Instance.MainRegistry;
        }
        #endregion
    }
}
